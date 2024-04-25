use victron_energy_api::Victron;

#[tokio::main]
async fn main() {
    // This example is taken directly from Victron's API documentation, and mixed with the usage of this library.
    // See https://vrm-api-docs.victronenergy.com/#/operations/auth/login
    // See https://vrm-api-docs.victronenergy.com/#/operations/users/me

    const USERNAME: &str = "john@example.com";
    const PASSWORD: &str = "somepassword";

    let victron = Victron::login(USERNAME, PASSWORD, None, false)
        .await
        .expect("Logged in successfully");

    let user = victron
        .get_user_info()
        .await
        .expect("Got user info successfully");

    println!("User: {:?}", user);
}
