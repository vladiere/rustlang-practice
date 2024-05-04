use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateParams<D> {
    pub data: D,
}

#[derive(Deserialize)]
pub struct UpdateParams<D> {
    pub id: i64,
    pub data: D,
}

#[derive(Deserialize)]
pub struct ListParams<F> {
    pub filter: Option<F>,
}

#[derive(Debug)]
pub struct GetParams {
    pub id: String,
}

#[derive(Deserialize)]
pub struct DeleteParams {
    pub id: i64,
}
