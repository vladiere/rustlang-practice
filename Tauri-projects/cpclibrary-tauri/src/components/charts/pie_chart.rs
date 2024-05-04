use charts_rs::PieChart;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PieChartComponent() -> impl IntoView {
    let mut pie_chart = PieChart::new(vec![
        ("rose 1", vec![40.0]).into(),
        ("rose 2", vec![38.0]).into(),
        ("rose 3", vec![32.0]).into(),
        ("rose 4", vec![30.0]).into(),
        ("rose 5", vec![28.0]).into(),
        ("rose 6", vec![26.0]).into(),
        ("rose 7", vec![22.0]).into(),
        ("rose 8", vec![18.0]).into(),
        ("rose 9", vec![14.0]).into(),
        ("rose 10", vec![10.0]).into(),
        ("rose 11", vec![35.0]).into(),
    ]);

    pie_chart.title_text = "Nightingale Chart".to_string();
    pie_chart.sub_title_text = "Fake Data".to_string();

    let parser = web_sys::DomParser::new().unwrap();
    let parse_element = parser
        .parse_from_string(
            &pie_chart.svg().unwrap(),
            web_sys::SupportedType::ImageSvgXml,
        )
        .unwrap();

    let window = web_sys::window().expect("no global 'window' exists");
    let document = window.document().expect("should have a document on window");

    spawn_local(async move {
        let div_chart = document
            .get_element_by_id("pie-chart")
            .expect("no div name pie-chart");
        let _ = div_chart.append_child(&parse_element.first_child().expect("empty first_child"));
    });
    view! {
        <Box class="bg-transparent">
            <div style="background-color: var(--box-bg-color)" id="pie-chart">
            </div>
        </Box>
    }
}
