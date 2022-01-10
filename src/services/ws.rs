use websocket::{Client, Message};
use websocket::client::request::Url;

fn stream {
    let url = Url::parse("ws://127.0.0.1:1234").unwrap(); // Get the URL
    let request = Client::connect(url).unwrap(); // Connect to the server
    let response = request.send().unwrap(); // Send the request
    response.validate().unwrap(); // Ensure the response is valid

    let client = response.begin(); // Get a Client

    let (mut sender, mut receiver) = client.split(); // Split the Client
    for message in receiver.incoming_messages() {
        let message: Message = message.unwrap();
        // Echo the message back
        sender.send_message(&message).unwrap();
    }
}