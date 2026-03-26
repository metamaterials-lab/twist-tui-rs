use std::ops::Rem;

use crate::model::Data;

use ratatui::prelude::*;
use ratatui::widgets::{Axis, Block, Chart, Dataset, Padding};

const COLOR_PALLETTE : [Color;4] = [
    Color::Red,
    Color::Cyan,
    Color::Blue,
    Color::Cyan
];

impl Widget for &Data {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let block = Block::bordered()
            .title("Torque vs. Angular Displacement")
            .padding(Padding::uniform(1));
        
        let mut datasets = Vec::new();
        let mut c = 0;
        for [data0,data1] in &self.data {
            datasets.push(
                Dataset::default()
                    .graph_type(ratatui::widgets::GraphType::Line)
                    .marker(symbols::Marker::Braille)
                    .data(data0)
                    .style( COLOR_PALLETTE[c] )
            );
            datasets.push(
                Dataset::default()
                    .graph_type(ratatui::widgets::GraphType::Line)
                    .marker(symbols::Marker::Braille)
                    .data(data1)
                    .style( COLOR_PALLETTE[c] )
            );
            c = (c + 1).rem( 4 );
        }


        let chart = Chart::new( datasets )
            .x_axis(
                Axis::default()
                    .title("Angle".white().style(Modifier::BOLD))
                    .bounds([self.x_lim.0, self.x_lim.1])
                    .labels( get_labels(self.x_lim).map(|x| x.light_cyan()) )
                    .blue()
            )
            .y_axis(
                Axis::default()
                    .title("Torque".white().style(Modifier::BOLD))
                    .bounds([self.y_lim.0, self.y_lim.1])
                    .labels( get_labels(self.y_lim).map(|x| x.light_cyan()) )
                    .blue()
            );

        chart.render(block.inner(area), buf);
        block.render(area, buf);
    }
}

fn get_labels( lim : (f64,f64) ) -> [String;4] {
    [
        format!("{:^6.2}", lim.0),
        format!("{:^6.2}", (lim.1+lim.0)*1.0/3.0),
        format!("{:^6.2}", (lim.1+lim.0)*2.0/3.0),
        format!("{:^6.2}", lim.1),
    ]
}
