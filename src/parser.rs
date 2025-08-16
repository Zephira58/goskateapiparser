// src/parser.rs

use chrono::{DateTime, Duration, FixedOffset, Utc};
use csv::ReaderBuilder;
use regex::{Regex, escape};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use crate::items;

#[derive(Debug, Deserialize)]
struct TradeRecord {
    #[serde(rename = "AuthorID")]
    author_id: u64,
    #[serde(rename = "Author")]
    author: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Content")]
    content: Option<String>,
    #[serde(rename = "Attachments")]
    attachments: Option<String>,
    #[serde(rename = "Reactions")]
    reactions: Option<String>,
}

#[derive(Debug, Default)]
struct ItemStats {
    prices: Vec<f64>,
    supply_posts: u32,
    demand_posts: u32,
    trade_dates: Vec<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize)]
struct EstimatedPrice {
    median: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,
}

#[derive(Debug, Serialize)]
struct SupplyDemand {
    supply_posts: u32,
    demand_posts: u32,
}

#[derive(Debug, Serialize)]
struct TradeChance {
    chance_to_buy: String,
    chance_to_sell: String,
}

#[derive(Debug, Serialize)]
struct ItemAnalysis {
    item: String,
    estimated_price: EstimatedPrice,
    supply_demand: SupplyDemand,
    estimated_trade_chances: TradeChance,
    rough_selling_frequency: String,
}

#[derive(Debug, Serialize)]
struct AnalysisOutput {
    total_parsing_time_ms: u128,
    overall_trade_data_span_days: f64,
    overall_trade_data_span_weeks: f64,
    overall_trade_data_span_months: f64,
    items: Vec<ItemAnalysis>,
}

pub fn run_trade_analysis(
    file_path: &str,
    is_verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    if is_verbose {
        println!("\n--- Starting Trade Analysis ---\n");
    }
    let start_time = Instant::now();

    if is_verbose {
        println!("Attempting to open CSV file: '{}'", file_path);
    }
    let file = File::open(file_path);
    let file = match file {
        Ok(f) => {
            if is_verbose {
                println!("Successfully opened CSV file.");
            }
            f
        }
        Err(e) => {
            eprintln!("ERROR: Could not open file '{}': {}", file_path, e);
            return Err(Box::new(e));
        }
    };
    let reader = BufReader::new(file);

    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let mut item_data: HashMap<String, ItemStats> = HashMap::new();
    let mut all_trade_dates: Vec<DateTime<FixedOffset>> = Vec::new();
    let mut processed_records_count = 0;
    let mut skipped_records_count = 0;

    if is_verbose {
        println!("Loading item keywords...");
    }
    let item_keywords = items::get_item_keywords();
    if is_verbose {
        println!("Item keywords loaded successfully.");
    }

    let price_regex = Regex::new(r"(\d[\d\.]*[kK]?|\d[\d,\.]*)").unwrap();
    let sell_regex = Regex::new(r"(?i)\b(sell|selling|wts)\b").unwrap();
    let buy_regex = Regex::new(r"(?i)\b(buy|buying|wtb)\b").unwrap();

    if is_verbose {
        println!("Starting to deserialize and process CSV records...");
    }
    for (i, result) in rdr.deserialize().enumerate() {
        let record: TradeRecord = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!(
                    "WARNING: Skipping malformed record on line {}: {}",
                    i + 2,
                    e
                );
                skipped_records_count += 1;
                continue;
            }
        };
        processed_records_count += 1;

        let content = if let Some(c) = record.content {
            c
        } else {
            if is_verbose {
                println!(
                    "Skipping record {} (Author: {}): Missing content.",
                    i + 2,
                    record.author
                );
            }
            skipped_records_count += 1;
            continue;
        };
        let content_lower = content.to_lowercase();

        let parsed_date = DateTime::parse_from_rfc3339(&record.date);
        let trade_date = match parsed_date {
            Ok(dt) => {
                all_trade_dates.push(dt);
                dt
            }
            Err(_) => {
                if is_verbose {
                    println!(
                        "Skipping record {} (Author: {}): Unparseable date format '{}'.",
                        i + 2,
                        record.author,
                        record.date
                    );
                }
                skipped_records_count += 1;
                continue;
            }
        };

        let mut found_item_name: Option<String> = None;
        for (item_name, regexes) in &item_keywords {
            for re in regexes {
                if re.is_match(&content_lower) {
                    found_item_name = Some(item_name.clone());
                    break;
                }
            }
            if found_item_name.is_some() {
                break;
            }
        }

        let item_name = match found_item_name {
            Some(name) => name,
            None => {
                if is_verbose {
                    println!(
                        "Skipping record {} (Author: {}): No identifiable item found in content.",
                        i + 2,
                        record.author
                    );
                }
                skipped_records_count += 1;
                continue;
            }
        };

        let price_str = price_regex.find(&content_lower);
        let price = if let Some(m) = price_str {
            let mut p_str = m.as_str().replace('$', "").replace(',', "");
            if p_str.ends_with('k') || p_str.ends_with('K') {
                p_str.pop();
                p_str.parse::<f64>().ok().map(|val| val * 1000.0)
            } else {
                p_str.parse::<f64>().ok()
            }
        } else {
            None
        };

        let price_val = match price {
            Some(p) => p,
            None => {
                if is_verbose {
                    println!(
                        "Skipping record {} (Author: {}): No valid price found for item '{}'.",
                        i + 2,
                        record.author,
                        item_name
                    );
                }
                skipped_records_count += 1;
                continue;
            }
        };

        let stats = item_data.entry(item_name).or_default();
        stats.prices.push(price_val);
        stats.trade_dates.push(trade_date);

        if sell_regex.is_match(&content_lower) {
            stats.supply_posts += 1;
        } else if buy_regex.is_match(&content_lower) {
            stats.demand_posts += 1;
        }
    }
    if is_verbose {
        println!(
            "Finished processing {} records ({} skipped).",
            processed_records_count, skipped_records_count
        );
    }

    let overall_parsing_time = start_time.elapsed();

    all_trade_dates.sort();

    let earliest_message_utc_epoch = all_trade_dates.first().map(|dt| dt.timestamp());
    let latest_message_utc_epoch = all_trade_dates.last().map(|dt| dt.timestamp());
    let parser_run_utc_epoch = Utc::now().timestamp();

    let total_duration = if all_trade_dates.len() > 1 {
        all_trade_dates
            .last()
            .unwrap()
            .signed_duration_since(*all_trade_dates.first().unwrap())
    } else {
        Duration::zero()
    };
    let total_days = total_duration.num_days() as f64;
    let total_weeks = total_duration.num_weeks() as f64;
    let total_months = total_days / 30.44;

    let data_display_period = if all_trade_dates.is_empty() {
        "No data available".to_string()
    } else if total_duration.num_seconds() == 0 {
        "Less than a day (or only one record)".to_string()
    } else if total_months >= 1.0 {
        let months = total_duration.num_days() / 30;
        let remaining_days = total_duration.num_days() % 30;
        format!("{} months, {} days", months, remaining_days)
    } else if total_weeks >= 1.0 {
        let weeks = total_duration.num_days() / 7;
        let remaining_days = total_duration.num_days() % 7;
        format!("{} weeks, {} days", weeks, remaining_days)
    } else {
        format!("{:.0} days", total_days)
    };

    if all_trade_dates.is_empty() {
        println!(
            "\nWARNING: No valid trade data found after parsing. Output will contain no item analysis."
        );
    }

    if is_verbose {
        println!("\nAggregating and sorting item data...");
    }
    let mut results: Vec<ItemAnalysis> = Vec::new();

    let mut sorted_item_data: Vec<(String, ItemStats)> = item_data.into_iter().collect();
    sorted_item_data.sort_by(|a, b| {
        let median_a = if !a.1.prices.is_empty() {
            let mut prices = a.1.prices.clone();
            prices.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
            let mid = prices.len() / 2;
            if prices.len() % 2 == 0 {
                (prices[mid - 1] + prices[mid]) / 2.0
            } else {
                prices[mid]
            }
        } else {
            0.0
        };
        let median_b = if !b.1.prices.is_empty() {
            let mut prices = b.1.prices.clone();
            prices.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
            let mid = prices.len() / 2;
            if prices.len() % 2 == 0 {
                (prices[mid - 1] + prices[mid]) / 2.0
            } else {
                prices[mid]
            }
        } else {
            0.0
        };
        median_b
            .partial_cmp(&median_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (item_name, mut stats) in sorted_item_data {
        stats
            .prices
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let median_price = if stats.prices.is_empty() {
            None
        } else {
            let mid = stats.prices.len() / 2;
            Some(if stats.prices.len() % 2 == 0 {
                (stats.prices[mid - 1] + stats.prices[mid]) / 2.0
            } else {
                stats.prices[mid]
            })
        };
        let min_price = stats.prices.first().cloned();
        let max_price = stats.prices.last().cloned();

        let total_posts = stats.supply_posts + stats.demand_posts;

        let buy_chance = if total_posts > 0 && stats.demand_posts > 0 {
            (stats.demand_posts as f64 / total_posts as f64) * 100.0
        } else {
            0.0
        };

        let sell_chance = if total_posts > 0 && stats.supply_posts > 0 {
            (stats.supply_posts as f64 / total_posts as f64) * 100.0
        } else {
            0.0
        };

        let frequency_str = if total_posts > 0 && total_days > 0.0 {
            let trades_per_day = total_posts as f64 / total_days;
            if trades_per_day >= 1.0 {
                format!("{:.2} times/day", trades_per_day)
            } else if trades_per_day * 7.0 >= 1.0 {
                format!("{:.2} times/week", trades_per_day * 7.0)
            } else if trades_per_day * 30.44 >= 1.0 {
                format!("{:.2} times/month", trades_per_day * 30.44)
            } else {
                format!("Once every {:.0} days", 1.0 / trades_per_day)
            }
        } else {
            "Infrequently/Not observed".to_string()
        };

        results.push(ItemAnalysis {
            item: item_name,
            estimated_price: EstimatedPrice {
                median: median_price,
                min: min_price,
                max: max_price,
            },
            supply_demand: SupplyDemand {
                supply_posts: stats.supply_posts,
                demand_posts: stats.demand_posts,
            },
            estimated_trade_chances: TradeChance {
                chance_to_buy: format!("{:.2}%", buy_chance),
                chance_to_sell: format!("{:.2}%", sell_chance),
            },
            rough_selling_frequency: frequency_str,
        });
    }
    if is_verbose {
        println!("Item data aggregation complete.");
    }

    let metadata_comments = format!(
        "# Trade Analysis Metadata\n\
        # ------------------------\n\
        # Earliest message (UTC Epoch): {}\n\
        # Latest message (UTC Epoch): {}\n\
        # Parser run time (UTC Epoch): {}\n\
        # CSV data time period: {}\n\
        # Total parsing and processing time: {} ms\n\
        # Overall trade data span: {:.2} days ({:.2} weeks, {:.2} months)\n\n",
        earliest_message_utc_epoch.map_or("N/A".to_string(), |e| e.to_string()),
        latest_message_utc_epoch.map_or("N/A".to_string(), |e| e.to_string()),
        parser_run_utc_epoch,
        data_display_period,
        overall_parsing_time.as_millis(),
        total_days,
        total_weeks,
        total_months
    );

    let final_output_struct = AnalysisOutput {
        total_parsing_time_ms: overall_parsing_time.as_millis(),
        overall_trade_data_span_days: total_days,
        overall_trade_data_span_weeks: total_weeks,
        overall_trade_data_span_months: total_months,
        items: results,
    };

    if is_verbose {
        println!("Serializing results to YAML format...");
    }
    let yaml_items_output = serde_yaml::to_string(&final_output_struct)?;
    if is_verbose {
        println!("YAML serialization complete.");
    }

    if is_verbose {
        println!("\n--- Trade Analysis Complete ---");
    }
    Ok(format!("{}{}", metadata_comments, yaml_items_output))
}
