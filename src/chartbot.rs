use chrono::{Utc, DateTime, DurationRound, Duration};
use eframe::{epi::App, egui::{CentralPanel, Ui, plot::{BoxElem, BoxSpread, BoxPlot, Plot, Legend, Line, Values, Value}, Color32, Stroke, InnerResponse, Response}};
use ftx::rest::{Rest, Candle, Resolution, Trade, GetHistoricalPrices};
// use futures::channel::mpsc::Receiver;
use rust_decimal::{Decimal, prelude::{ToPrimitive, FromPrimitive}};
use tokio::sync::mpsc::Receiver;

pub struct ChartBot {
    pub resolution: Resolution,
    // pub rx: Receiver<Trade>,
    // pub client: Rest,
    pub candles: Vec<BoxElem>,
    // pub cur_candle: Candle,
    pub draw_mode: bool,
    pub pointer_coord: Option<(f64, f64)>,
    pub current_line: Option<((f64, f64), (f64, f64))>,
    pub lines: Vec<((f64, f64), (f64, f64))>,
}

impl ChartBot {
    pub async fn new(resolution: Resolution) -> Self {

        // let candles = client.request(GetHistoricalPrices::new_paged(
        //     "BTC/USD", resolution, Some(26280), Some(Utc::now()-Duration::seconds(26280*resolution.get_seconds() as i64)), Some(current_trunc(&resolution)-Duration::seconds(1)))).await.unwrap();

        // println!("{}", candles.len());
        let file = std::fs::File::open("btcusd.csv").unwrap();
        let mut recs = csv::Reader::from_reader(file);
        let recs = recs.deserialize();
        let mut candles = Vec::new();
        for rec in recs {
            let record: Candle = rec.unwrap();
            candles.push(record);
        }
        let mut boxdata = Vec::new();
        
        for candle in candles {
            boxdata.push(BoxElem::new(
                candle.start_time.timestamp() as f64/resolution.get_seconds() as f64 * 40.0,
                BoxSpread::new(
                    candle.low.to_f64().unwrap(),
                    (std::cmp::min(candle.open, candle.close)).to_f64().unwrap(),
                    (candle.open+candle.close).to_f64().unwrap()/2.0,
                    (std::cmp::max(candle.open, candle.close)).to_f64().unwrap(),
                    candle.high.to_f64().unwrap(),
                ),
                ).fill({
                    if candle.close > candle.open {
                        Color32::from_rgb(0,255,0)
                    } else {
                        Color32::from_rgb(255,0,0)
                    }
                }).whisker_width(0.)
                .box_width(0.7*40.0)
                .stroke(Stroke {
                    width: 0.05,
                    color: {
                        if candle.close > candle.open {
                            Color32::from_rgb(0,255,0)
                        } else {
                            Color32::from_rgb(255,0,0)
                        }
                    }
                }),
            );
            
        }
        
        
        // let curcandle = { 
        //     let vec = client.request(GetHistoricalPrices::new_paged(
        //     "BTC/USD",
        //     resolution,
        //     Some(1),
        //     None,
        //     None,
        // )).await.unwrap();
        // if vec.len() == 0 {
        //     let last_close = candles[candles.len()-1].close;
        //     Candle {
        //         start_time: current_trunc(&resolution),
        //         open: last_close,
        //         high: last_close,
        //         low: last_close,
        //         close: last_close,
        //         volume: Decimal::from_u32(0).unwrap(),
        //     }
        // } else {
        //     vec[0]
        // }
    // };
        Self {
            // client,
            resolution,
            // rx,
            // cur_candle: curcandle,
            candles: boxdata,
            draw_mode: false,
            pointer_coord: None,
            current_line: None,
            lines: Vec::new(),
        }
    }

    fn candlesticks(&mut self, ui: &mut Ui) -> Response {
        let boxdata = self.candles.clone();

        // let cand = self.cur_candle;

        // candles.push(cand);

        

        let chart = BoxPlot::new(boxdata).vertical();

        let plot = Plot::new("Candlestick Chart")
            .legend(Legend::default())
            .data_aspect(1.0)
            .show_axes([false, false])
            .show_background(true)
            ;

        let InnerResponse {
            response,
            inner: bounds
        } = plot.show(ui, |plot_ui| {
            (
                plot_ui.box_plot(chart),
                if self.lines.len() > 0 {
                    for index in 0..self.lines.len() {
                        plot_ui.line(
                            straight_line(self.lines[index])
                        );
                    }
                    
                },
                plot_ui.pointer_coordinate(),
                match self.current_line {
                    Some(((x1, y1), (x2, y2))) => {
                        plot_ui.line(
                            straight_line(((x1, y1), (x2, y2)))
                        );
                    },
                    None => {},
                },
            )
        });

        match bounds.2 {
            Some(coord) => {
                self.pointer_coord = Some((coord.x, coord.y));
            },
            None => {},
        }
        response

    }
    
}

impl App for ChartBot {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // while let Ok(trade) = self.rx.try_recv() {
            //     self.cur_candle = update_candle(self.cur_candle, trade);
            // }

            if ui.button("Draw line").clicked() {
                self.draw_mode = !self.draw_mode;
            }

            let chart = self.candlesticks(ui);

            if self.draw_mode {
                match self.current_line {
                    Some(((x1, y1), (_x2, _y2))) => {
                        if chart.clicked() {
                            self.lines.push(((x1, y1), self.pointer_coord.unwrap()));
                            self.current_line = None;   
                        } else {
                            self.current_line = Some(((x1, y1), self.pointer_coord.unwrap()));
                        }
                        
                    },
                    None => {
                        if chart.clicked() {
                            self.current_line = Some((self.pointer_coord.unwrap(), self.pointer_coord.unwrap()));
                        }
                    }
                }
            };
            

            // if current_trunc(&self.resolution) == self.cur_candle.start_time {

            // } else {
            //     self.candles.push(self.cur_candle);
            //     self.cur_candle = {
            //         Candle {
            //             start_time: current_trunc(&self.resolution),
            //             open: self.cur_candle.close,
            //             high: self.cur_candle.close,
            //             low: self.cur_candle.close,
            //             close: self.cur_candle.close,
            //             volume: Decimal::from_u32(0).unwrap(),
            //         }
            //     }
            // }

            ctx.request_repaint();
        });
    }

    fn setup(&mut self, _ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame, _storage: Option<&dyn eframe::epi::Storage>) {
        
    }

    fn name(&self) -> &str {
        "ChartBot"
    }
}

pub fn straight_line(tuple: ((f64, f64), (f64, f64))) -> Line {
    Line::new(
        Values::from_values(
            vec![
                Value::new(tuple.0.0, tuple.0.1),
                Value::new(tuple.1.0, tuple.1.1),
            ]
        ),
    )
}

pub fn current_trunc(resolution: &Resolution) -> DateTime<Utc>{
    let now = chrono::Utc::now();
    let trunced = match resolution {
        Resolution::FifteenSeconds => now.duration_trunc(Duration::seconds(15)).unwrap(),
        Resolution::Minute => now.duration_trunc(Duration::minutes(1)).unwrap(),
        Resolution::FiveMinutes => now.duration_trunc(Duration::minutes(5)).unwrap(),
        Resolution::FifteenMinutes => now.duration_trunc(Duration::minutes(15)).unwrap(),
        Resolution::Hour => now.duration_trunc(Duration::hours(1)).unwrap(),
        Resolution::FourHours => now.duration_trunc(Duration::hours(4)).unwrap(),
        Resolution::Day => now.duration_trunc(Duration::days(1)).unwrap(),
        Resolution::TwoDays => now.duration_trunc(Duration::days(2)).unwrap(),
        Resolution::ThreeDays => now.duration_trunc(Duration::days(3)).unwrap(),
        Resolution::FourDays => now.duration_trunc(Duration::days(4)).unwrap(),
        Resolution::FiveDays => now.duration_trunc(Duration::days(5)).unwrap(),
        Resolution::SixDays => now.duration_trunc(Duration::days(6)).unwrap(),
        Resolution::Week => now.duration_trunc(Duration::days(7)).unwrap(),
        Resolution::EightDays => now.duration_trunc(Duration::days(8)).unwrap(),
        Resolution::NineDays => now.duration_trunc(Duration::days(9)).unwrap(),
        Resolution::TenDays => now.duration_trunc(Duration::days(10)).unwrap(),
        Resolution::ElevenDays => now.duration_trunc(Duration::days(11)).unwrap(),
        Resolution::TwelveDays => now.duration_trunc(Duration::days(12)).unwrap(),
        Resolution::ThirteenDays => now.duration_trunc(Duration::days(13)).unwrap(),
        Resolution::FourteenDays => now.duration_trunc(Duration::days(14)).unwrap(),
        Resolution::FifteenDays => now.duration_trunc(Duration::days(15)).unwrap(),
        Resolution::SixteenDays => now.duration_trunc(Duration::days(16)).unwrap(),
        Resolution::SeventeenDays => now.duration_trunc(Duration::days(17)).unwrap(),
        Resolution::EighteenDays => now.duration_trunc(Duration::days(18)).unwrap(),
        Resolution::NineteenDays => now.duration_trunc(Duration::days(19)).unwrap(),
        Resolution::TwentyDays => now.duration_trunc(Duration::days(20)).unwrap(),
        Resolution::TwentyOneDays => now.duration_trunc(Duration::days(21)).unwrap(),
        Resolution::TwentyTwoDays => now.duration_trunc(Duration::days(22)).unwrap(),
        Resolution::TwentyThreeDays => now.duration_trunc(Duration::days(23)).unwrap(),
        Resolution::TwentyFourDays => now.duration_trunc(Duration::days(24)).unwrap(),
        Resolution::TwentyFiveDays => now.duration_trunc(Duration::days(25)).unwrap(),
        Resolution::TwentySixDays => now.duration_trunc(Duration::days(26)).unwrap(),
        Resolution::TwentySevenDays => now.duration_trunc(Duration::days(27)).unwrap(),
        Resolution::TwentyEightDays => now.duration_trunc(Duration::days(28)).unwrap(),
        Resolution::TwentyNineDays => now.duration_trunc(Duration::days(29)).unwrap(),
        Resolution::ThirtyDays => now.duration_trunc(Duration::days(30)).unwrap(),
    };
    trunced
}

fn update_candle(cand: Candle, trade: Trade) -> Candle {
    Candle {
        open: cand.open,
        close: trade.price,
        high: cand.high.max(trade.price),
        low: cand.low.min(trade.price),
        volume: cand.volume + trade.size,
        start_time: cand.start_time,
    }
}