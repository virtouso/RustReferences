pub struct Player {
    pub player_id: String,
    pub player_google_data: PlayerGoogleData,
}


struct PlayerGoogleData {
    pub email: String,
    pub given_name: String,
    pub family_name: String
}