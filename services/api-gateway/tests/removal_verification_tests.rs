use std::fs;
use std::path::Path;

#[test]
fn test_p2p_context_removed() {
    let p2p_path = Path::new("src/bounded_contexts/p2p");
    assert!(!p2p_path.exists(), "P2P context should be removed");
}

#[test]
fn test_federation_context_removed() {
    let federation_path = Path::new("src/bounded_contexts/federation");
    assert!(!federation_path.exists(), "Federation context should be removed");
}

#[test]
fn test_monitoring_context_removed() {
    let monitoring_path = Path::new("src/bounded_contexts/monitoring");
    assert!(!monitoring_path.exists(), "Monitoring context should be removed");
}

#[test]
fn test_core_contexts_preserved() {
    let core_contexts = vec![
        "src/bounded_contexts/music",
        "src/bounded_contexts/user", 
        "src/bounded_contexts/payment",
        "src/bounded_contexts/campaign",
        "src/bounded_contexts/listen_reward",
        "src/bounded_contexts/fractional_ownership",
    ];
    
    for context in core_contexts {
        assert!(Path::new(context).exists(), "Core context {} should be preserved", context);
    }
}

#[test]
fn test_dependencies_cleaned() {
    let cargo_content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    
    // Check that problematic dependencies are removed
    let problematic_deps = vec![
        "webrtc-rs",
        "activitypub", 
        "p2p-libp2p",
        "libp2p",
    ];
    
    for dep in problematic_deps {
        assert!(
            !cargo_content.contains(dep),
            "Problematic dependency {} should be removed from Cargo.toml",
            dep
        );
    }
    
    // Check that core dependencies are preserved
    let core_deps = vec![
        "tokio",
        "serde",
        "sqlx",
        "axum",
        "tower",
    ];
    
    for dep in core_deps {
        assert!(
            cargo_content.contains(dep),
            "Core dependency {} should be preserved in Cargo.toml",
            dep
        );
    }
}

#[test]
fn test_mod_rs_updated() {
    let mod_content = fs::read_to_string("src/bounded_contexts/mod.rs")
        .expect("Failed to read mod.rs");
    
    // Check that problematic modules are removed
    let problematic_modules = vec!["p2p", "federation", "monitoring"];
    
    for module in problematic_modules {
        assert!(
            !mod_content.contains(&format!("mod {}", module)),
            "Problematic module {} should be removed from mod.rs",
            module
        );
    }
    
    // Check that core modules are preserved
    let core_modules = vec!["music", "user", "payment", "campaign", "listen_reward", "fractional_ownership"];
    
    for module in core_modules {
        assert!(
            mod_content.contains(&format!("mod {}", module)),
            "Core module {} should be preserved in mod.rs",
            module
        );
    }
} 