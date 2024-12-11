use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "SmartConnect Wi-Fi Router".to_string(),
            price: 49.99,
            description: "Upgrade your home network with the SmartConnect Wi-Fi Router. Offering lightning-fast speeds and wide coverage, this device ensures seamless streaming, gaming, and browsing for all your devices.".to_string(),
            image: "/router.png".to_string()
        },
        Product {
            id: 2,
            name: "HyperCharge Power Bank".to_string(),
            price: 29.99,
            description: "Stay powered on the go with the HyperCharge Power Bank. Featuring fast charging capabilities and a sleek design, it's perfect for smartphones, tablets, and other gadgets.".to_string(),
            image: "/powerbank.png".to_string()
        },
        Product {
            id: 3,
            name: "PixelSharp 4K Monitor".to_string(),
            price: 299.99,
            description: "Enhance your workspace with the PixelSharp 4K Monitor. With stunning resolution and vibrant colors, this monitor is ideal for productivity and entertainment.".to_string(),
            image: "/monitor.png".to_string()
        },
        Product {
            id: 4,
            name: "SonicBeats Wireless Headphones".to_string(),
            price: 99.99,
            description: "Experience unparalleled sound quality with the SonicBeats Wireless Headphones. Featuring noise cancellation and long battery life for all-day listening.".to_string(),
            image: "/headphones.png".to_string()
        },
        Product {
            id: 5,
            name: "UltraTouch Smartwatch".to_string(),
            price: 199.99,
            description: "Track your health and stay connected with the UltraTouch Smartwatch. Packed with features like fitness tracking, notifications, and customizable watch faces.".to_string(),
            image: "/smartwatch.png".to_string()
        },
        Product {
            id: 6,
            name: "GameMaster RGB Keyboard".to_string(),
            price: 69.99,
            description: "Elevate your gaming setup with the GameMaster RGB Keyboard. Featuring customizable lighting and responsive keys for a competitive edge.".to_string(),
            image: "/keyboard.png".to_string()
        }
    ]
}
