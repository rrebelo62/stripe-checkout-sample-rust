use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CreateCheckoutSessionResponse{
	pub session_id:String,
}
