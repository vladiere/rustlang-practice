use super::MyData;
use leptonic::prelude::*;
use leptos::*;
use leptos_chartistry::*;
use leptos_meta::Style;

#[component]
pub fn LineChartComponent(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Lines are added to the series
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("Petterson Biography"))
        .line(
            Line::new(|data: &MyData| data.y2)
                .with_name("History of the philippines English version"),
        )
        .line(Line::new(|data: &MyData| data.y3).with_name("The pepper in the toilet"))
        .line(Line::new(|data: &MyData| data.y4).with_name("The black History of the world"))
        .line(Line::new(|data: &MyData| data.y5).with_name("Java Data Structures and Algorithm"));
    view! {
        <Style>
            "._chartistry_tooltip {
                background-color: var(--box-bg-color) !important;
                border: 1px solid var(--box-border-color) !important;
            }
            ._chartistry_grid_line_x, ._chartistry_grid_line_y {
                stroke: var(--box-border-color);
            }
            ._chartistry_snippet {
                display: flex;
                flex-direction: row;
                align-items: center;
                gap: 5px;
            }"
        </Style>

        <Box style="background-color: var(--box-bg-color); fill: var(--chart-fill-color);" class="rounded-lg shadow-lg p-5">
            <Chart
                aspect_ratio=AspectRatio::from_outer_height(485.0, 1.3)
                debug=debug
                series=series
                data=data

                // Decorate our chart
                top=RotatedLabel::middle("Top 5 Most Borrowed Books")
                left=TickLabels::aligned_floats()
                // bottom=Legend::end()
                inner=[
                    // Standard set of inner layout options
                    AxisMarker::left_edge().into_inner(),
                    AxisMarker::bottom_edge().into_inner(),
                    XGridLine::default().into_inner(),
                    YGridLine::default().into_inner(),
                    YGuideLine::over_mouse().into_inner(),
                    XGuideLine::over_data().into_inner(),
                ]
                tooltip=Tooltip::left_cursor().show_x_ticks(false)
            />
        </Box>
    }
}
