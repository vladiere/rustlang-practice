use super::MyData;
use leptonic::prelude::*;
use leptos::*;
use leptos_chartistry::*;
use leptos_meta::Style;

#[component]
pub fn BarChartComponent(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .bar(Bar::new(|data: &MyData| data.y1).with_name("BSIT"))
        .bar(Bar::new(|data: &MyData| data.y2).with_name("BEED"))
        .bar(Bar::new(|data: &MyData| data.y3).with_name("BSED"))
        .bar(Bar::new(|data: &MyData| data.y4).with_name("BSHM"))
        .bar(Bar::new(|data: &MyData| data.y5).with_name("FACULTY"));
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

                top=RotatedLabel::middle("Most Online Department")
                left=TickLabels::aligned_floats()
                inner=[
                    AxisMarker::left_edge().into_inner(),
                    AxisMarker::bottom_edge().into_inner(),
                    YGridLine::default().into_inner(),
                    YGuideLine::over_mouse().into_inner(),
                    XGuideLine::over_data().into_inner(),
                ]
                tooltip=Tooltip::left_cursor().show_x_ticks(false)
            />
        </Box>
    }
}
