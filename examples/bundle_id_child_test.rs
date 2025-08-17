//! Child process test for bundle identifier override detection

use candle_coreml::CacheManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Child Process Bundle ID Test");

    let manager = CacheManager::new()?;
    println!("Bundle ID in child: {:?}", manager.bundle_identifier());

    // Check environment variables
    println!(
        "CFBundleIdentifier: {:?}",
        std::env::var("CFBundleIdentifier")
    );
    println!("BUNDLE_ID: {:?}", std::env::var("BUNDLE_ID"));

    Ok(())
}
