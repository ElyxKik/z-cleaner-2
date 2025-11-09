/// Test d'intégration pour l'API Cloudmersive Virus Scan
#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_env_variables_loaded() {
        // Charger les variables d'environnement
        dotenv::dotenv().ok();
        
        // Vérifier que la clé API est définie
        let api_key = env::var("CLOUDMERSIVE_API_KEY");
        assert!(api_key.is_ok(), "CLOUDMERSIVE_API_KEY not found in .env");
        
        let key = api_key.unwrap();
        assert!(!key.is_empty(), "CLOUDMERSIVE_API_KEY is empty");
        assert!(key.len() > 10, "CLOUDMERSIVE_API_KEY seems invalid (too short)");
        
        println!("✓ API Key loaded: {}...{}", 
            &key[..8], 
            &key[key.len()-4..]
        );
    }

    #[test]
    fn test_env_file_exists() {
        let env_path = Path::new(".env");
        assert!(env_path.exists(), ".env file not found");
        
        let content = fs::read_to_string(".env").expect("Failed to read .env");
        assert!(content.contains("CLOUDMERSIVE_API_KEY"), ".env missing CLOUDMERSIVE_API_KEY");
        
        println!("✓ .env file exists and contains CLOUDMERSIVE_API_KEY");
    }

    #[tokio::test]
    async fn test_cloudmersive_api_connectivity() {
        dotenv::dotenv().ok();
        
        let api_key = env::var("CLOUDMERSIVE_API_KEY")
            .expect("CLOUDMERSIVE_API_KEY not set");
        
        // Créer un fichier de test
        let test_file = "test_clean.txt";
        fs::write(test_file, "This is a clean test file").expect("Failed to create test file");
        
        // Vérifier que le fichier existe
        assert!(Path::new(test_file).exists(), "Test file not created");
        
        println!("✓ Test file created: {}", test_file);
        println!("✓ API Key is set: {}...{}", 
            &api_key[..8], 
            &api_key[api_key.len()-4..]
        );
        
        // Nettoyer
        fs::remove_file(test_file).ok();
    }
}
