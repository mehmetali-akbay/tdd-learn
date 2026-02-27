use structs::*;

const EPS: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPS,
        "expected {expected}, got {actual}"
    );
}

// ===== Topic 1: Named-field structs =====

#[test]
fn build_user_sets_defaults_and_extra_fields() {
    let user = build_user("a@b.com".to_string(), "alice".to_string());

    assert!(user.active);
    assert_eq!(user.username, "alice");
    assert_eq!(user.email, "a@b.com");
    assert_eq!(user.sign_in_count, 1);
    assert_eq!(user.display_name, "alice");
    assert_eq!(user.roles, vec!["user"]);
}

#[test]
fn default_guest_has_expected_values() {
    let guest = default_guest();

    assert_eq!(guest.username, "guest");
    assert_eq!(guest.email, "guest@example.com");
    assert_eq!(guest.display_name, "Guest User");
    assert_eq!(guest.sign_in_count, 0);
    assert!(guest.active);
    assert_eq!(guest.roles, vec!["guest"]);
}

#[test]
fn rename_updates_only_username() {
    let mut user = build_user("mail@example.com".to_string(), "before".to_string());
    user.rename("after".to_string());

    assert_eq!(user.username, "after");
    assert_eq!(user.display_name, "before");
}

#[test]
fn set_display_name_changes_label_output() {
    let mut user = build_user("mail@example.com".to_string(), "alice".to_string());
    user.set_display_name("Alice A.".to_string());

    assert_eq!(user.contact_label(), "Alice A. <mail@example.com>");
}

#[test]
fn deactivate_and_activate_roundtrip() {
    let mut user = build_user("a@b.com".to_string(), "alice".to_string());

    user.deactivate();
    assert!(!user.active);

    user.activate();
    assert!(user.active);
}

#[test]
fn sign_in_increments_counter() {
    let mut user = default_guest();
    user.sign_in();
    user.sign_in();

    assert_eq!(user.sign_in_count, 2);
}

#[test]
fn grant_role_normalizes_and_deduplicates() {
    let mut user = build_user("a@b.com".to_string(), "alice".to_string());

    user.grant_role(" Admin ".to_string());
    user.grant_role("ADMIN".to_string());
    user.grant_role("".to_string());

    assert_eq!(user.roles, vec!["admin", "user"]);
}

#[test]
fn has_role_is_case_insensitive() {
    let mut user = default_guest();
    user.grant_role("Editor".to_string());

    assert!(user.has_role("editor"));
    assert!(user.has_role("EDITOR"));
    assert!(!user.has_role("owner"));
}

// ===== Topic 2: Ownership with struct fields =====

#[test]
fn consume_username_moves_user_and_returns_name() {
    let user = build_user("a@b.com".to_string(), "alice".to_string());
    assert_eq!(consume_username(user), "alice");
}

#[test]
fn duplicate_email_borrows_without_moving() {
    let user = build_user("x@y.com".to_string(), "bob".to_string());
    let cloned = duplicate_email(&user);

    assert_eq!(cloned, "x@y.com");
    assert_eq!(user.email, "x@y.com");
}

#[test]
fn user_with_new_email_copies_other_fields() {
    let mut original = build_user("old@example.com".to_string(), "carol".to_string());
    original.grant_role("admin".to_string());

    let updated = user_with_new_email(&original, "new@example.com".to_string());

    assert_eq!(updated.email, "new@example.com");
    assert_eq!(updated.username, "carol");
    assert_eq!(updated.roles, original.roles);
    assert_eq!(updated.display_name, original.display_name);
}

#[test]
fn user_with_new_email_does_not_mutate_original() {
    let original = build_user("old@example.com".to_string(), "carol".to_string());
    let _updated = user_with_new_email(&original, "new@example.com".to_string());

    assert_eq!(original.email, "old@example.com");
}

// ===== Topic 3: Struct update syntax =====

#[test]
fn clone_with_username_keeps_original_unchanged() {
    let original = build_user("old@example.com".to_string(), "old".to_string());
    let cloned = clone_with_username(&original, "new".to_string());

    assert_eq!(original.username, "old");
    assert_eq!(cloned.username, "new");
    assert_eq!(cloned.email, original.email);
}

#[test]
fn clone_with_username_deep_clones_roles() {
    let original = build_user("x@y.com".to_string(), "name".to_string());
    let mut cloned = clone_with_username(&original, "new".to_string());
    cloned.grant_role("admin".to_string());

    assert!(!original.has_role("admin"));
    assert!(cloned.has_role("admin"));
}

#[test]
fn move_with_username_reuses_remaining_fields() {
    let original = User {
        active: false,
        username: "start".to_string(),
        email: "x@y.com".to_string(),
        sign_in_count: 42,
        display_name: "Start Name".to_string(),
        roles: vec!["owner".to_string()],
    };

    let moved = move_with_username(original, "end".to_string());

    assert_eq!(moved.username, "end");
    assert_eq!(moved.email, "x@y.com");
    assert_eq!(moved.display_name, "Start Name");
    assert_eq!(moved.roles, vec!["owner"]);
    assert!(!moved.active);
    assert_eq!(moved.sign_in_count, 42);
}

#[test]
fn move_with_username_keeps_roles_and_display_name() {
    let user = build_user("a@b.com".to_string(), "alice".to_string());
    let moved = move_with_username(user, "bob".to_string());

    assert_eq!(moved.display_name, "alice");
    assert!(moved.has_role("user"));
}

// ===== Topic 4: Tuple structs and unit-like structs =====

#[test]
fn meters_to_mm_converts_units() {
    assert_eq!(meters_to_mm(Meters(3)), Millimeters(3000));
}

#[test]
fn add_mm_adds_values() {
    assert_eq!(add_mm(Millimeters(500), Millimeters(250)), Millimeters(750));
}

#[test]
fn add_mm_uses_saturating_addition() {
    assert_eq!(
        add_mm(Millimeters(u32::MAX), Millimeters(1)),
        Millimeters(u32::MAX)
    );
}

#[test]
fn user_id_to_string_is_readable() {
    assert_eq!(user_id_to_string(UserId(15)), "UserId(15)");
}

#[test]
fn always_equal_is_constructible() {
    let _a = AlwaysEqual;
    let _b = AlwaysEqual;
}

// ===== Topic 5: Rectangle from the Rust Book =====

#[test]
fn area_tuple_and_struct_paths_match() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    assert_eq!(area_tuple((30, 50)), 1500);
    assert_eq!(area_rect(&rect), 1500);
    assert_eq!(rect.area(), 1500);
}

#[test]
fn width_returns_true_only_when_positive() {
    let a = Rectangle {
        width: 1,
        height: 2,
    };
    let b = Rectangle {
        width: 0,
        height: 2,
    };

    assert!(a.width());
    assert!(!b.width());
}

#[test]
fn can_hold_is_strictly_larger_in_both_dimensions() {
    let big = Rectangle {
        width: 8,
        height: 7,
    };
    let small = Rectangle {
        width: 5,
        height: 1,
    };
    let equal = Rectangle {
        width: 8,
        height: 7,
    };

    assert!(big.can_hold(&small));
    assert!(!small.can_hold(&big));
    assert!(!big.can_hold(&equal));
}

#[test]
fn square_builds_equal_sides() {
    let sq = Rectangle::square(6);
    assert_eq!(sq.width, 6);
    assert_eq!(sq.height, 6);
    assert!(sq.is_square());
}

#[test]
fn scale_mutates_dimensions() {
    let mut rect = Rectangle {
        width: 3,
        height: 4,
    };

    rect.scale(5);

    assert_eq!(rect.width, 15);
    assert_eq!(rect.height, 20);
}

#[test]
fn perimeter_and_diagonal_are_correct() {
    let rect = Rectangle {
        width: 3,
        height: 4,
    };

    assert_eq!(rect.perimeter(), 14);
    assert_close(rect.diagonal(), 5.0);
}

#[test]
fn is_square_distinguishes_shapes() {
    assert!(Rectangle::square(9).is_square());
    assert!(
        !Rectangle {
            width: 9,
            height: 8
        }
        .is_square()
    );
}

#[test]
fn fits_inside_without_rotation_respects_orientation() {
    let a = Rectangle {
        width: 4,
        height: 6,
    };
    let b = Rectangle {
        width: 6,
        height: 4,
    };

    assert!(!a.fits_inside(&b, false));
}

#[test]
fn fits_inside_with_rotation_can_succeed() {
    let a = Rectangle {
        width: 4,
        height: 6,
    };
    let b = Rectangle {
        width: 6,
        height: 4,
    };

    assert!(a.fits_inside(&b, true));
}

// ===== Topic 6: Complex builder/state challenge =====

#[test]
fn builder_reports_missing_id() {
    let result = AccountBuilder::new()
        .owner("Alice".to_string())
        .email("alice@example.com".to_string())
        .build();

    assert_eq!(result, Err(AccountBuildError::MissingId));
}

#[test]
fn builder_reports_missing_owner() {
    let result = AccountBuilder::new()
        .id(UserId(1))
        .email("alice@example.com".to_string())
        .build();

    assert_eq!(result, Err(AccountBuildError::MissingOwner));
}

#[test]
fn builder_reports_missing_email() {
    let result = AccountBuilder::new()
        .id(UserId(1))
        .owner("Alice".to_string())
        .build();

    assert_eq!(result, Err(AccountBuildError::MissingEmail));
}

#[test]
fn builder_rejects_invalid_owner() {
    let result = AccountBuilder::new()
        .id(UserId(1))
        .owner("   ".to_string())
        .email("alice@example.com".to_string())
        .build();

    assert_eq!(result, Err(AccountBuildError::InvalidOwner));
}

#[test]
fn builder_rejects_invalid_email() {
    let result = AccountBuilder::new()
        .id(UserId(1))
        .owner("Alice".to_string())
        .email("invalid".to_string())
        .build();

    assert_eq!(result, Err(AccountBuildError::InvalidEmail));
}

#[test]
fn builder_rejects_invalid_tag() {
    let result = AccountBuilder::new()
        .id(UserId(1))
        .owner("Alice".to_string())
        .email("alice@example.com".to_string())
        .add_tag("   ".to_string())
        .build();

    assert_eq!(result, Err(AccountBuildError::InvalidTag));
}

#[test]
fn builder_sets_defaults_for_optional_fields() {
    let account = AccountBuilder::new()
        .id(UserId(7))
        .owner("Alice".to_string())
        .email("alice@example.com".to_string())
        .build()
        .unwrap();

    assert_eq!(account.id(), UserId(7));
    assert_eq!(account.owner(), "Alice");
    assert_eq!(account.email(), "alice@example.com");
    assert!(account.is_active());
    assert_eq!(account.login_count(), 0);
    assert_eq!(account.tier(), AccountTier::Free);
    assert_eq!(account.created_at_epoch(), 0);
    assert_eq!(account.last_login_ip(), None);
    assert!(account.tags().is_empty());
}

#[test]
fn builder_applies_optional_fields_and_dedups_tags() {
    let account = AccountBuilder::new()
        .id(UserId(8))
        .owner("Bob".to_string())
        .email("bob@example.com".to_string())
        .tier(AccountTier::Pro)
        .created_at_epoch(1_700_000_000)
        .add_tag("Admin".to_string())
        .add_tag("admin".to_string())
        .add_tag("ops".to_string())
        .build()
        .unwrap();

    assert_eq!(account.tier(), AccountTier::Pro);
    assert_eq!(account.created_at_epoch(), 1_700_000_000);
    assert_eq!(account.tags(), &["admin".to_string(), "ops".to_string()]);
}

#[test]
fn record_login_updates_count_and_ip() {
    let mut account = AccountBuilder::new()
        .id(UserId(2))
        .owner("Bob".to_string())
        .email("bob@example.com".to_string())
        .build()
        .unwrap();

    account.record_login("10.0.0.1".to_string()).unwrap();
    account.record_login("10.0.0.2".to_string()).unwrap();

    assert_eq!(account.login_count(), 2);
    assert_eq!(account.last_login_ip(), Some("10.0.0.2"));
}

#[test]
fn record_login_requires_active_account() {
    let mut account = AccountBuilder::new()
        .id(UserId(3))
        .owner("Carol".to_string())
        .email("carol@example.com".to_string())
        .build()
        .unwrap();

    account.deactivate();

    assert_eq!(
        account.record_login("10.0.0.1".to_string()),
        Err(AccountActionError::Inactive)
    );
    assert_eq!(account.login_count(), 0);
}

#[test]
fn record_login_rejects_invalid_ip() {
    let mut account = AccountBuilder::new()
        .id(UserId(4))
        .owner("Dan".to_string())
        .email("dan@example.com".to_string())
        .build()
        .unwrap();

    assert_eq!(
        account.record_login("   ".to_string()),
        Err(AccountActionError::InvalidIp)
    );
    assert_eq!(account.login_count(), 0);
}

#[test]
fn change_email_accepts_valid_input() {
    let mut account = AccountBuilder::new()
        .id(UserId(5))
        .owner("Eve".to_string())
        .email("eve@example.com".to_string())
        .build()
        .unwrap();

    assert_eq!(account.change_email("new@example.com".to_string()), Ok(()));
    assert_eq!(account.email(), "new@example.com");
}

#[test]
fn change_email_rejects_invalid_input_and_keeps_previous() {
    let mut account = AccountBuilder::new()
        .id(UserId(5))
        .owner("Eve".to_string())
        .email("eve@example.com".to_string())
        .build()
        .unwrap();

    assert_eq!(
        account.change_email("invalid".to_string()),
        Err(AccountActionError::InvalidEmail)
    );
    assert_eq!(account.email(), "eve@example.com");
}

#[test]
fn add_tag_dedups_and_normalizes() {
    let mut account = AccountBuilder::new()
        .id(UserId(6))
        .owner("Frank".to_string())
        .email("frank@example.com".to_string())
        .build()
        .unwrap();

    account.add_tag("  Ops  ".to_string()).unwrap();
    account.add_tag("ops".to_string()).unwrap();

    assert_eq!(account.tags(), &["ops".to_string()]);
}

#[test]
fn add_tag_rejects_invalid_tag() {
    let mut account = AccountBuilder::new()
        .id(UserId(6))
        .owner("Frank".to_string())
        .email("frank@example.com".to_string())
        .build()
        .unwrap();

    assert_eq!(
        account.add_tag("   ".to_string()),
        Err(AccountActionError::InvalidTag)
    );
}

#[test]
fn remove_tag_returns_true_when_present() {
    let mut account = AccountBuilder::new()
        .id(UserId(7))
        .owner("Grace".to_string())
        .email("grace@example.com".to_string())
        .add_tag("ops".to_string())
        .build()
        .unwrap();

    assert!(account.remove_tag("ops"));
    assert!(!account.has_tag("ops"));
}

#[test]
fn remove_tag_returns_false_when_missing() {
    let mut account = AccountBuilder::new()
        .id(UserId(8))
        .owner("Helen".to_string())
        .email("helen@example.com".to_string())
        .build()
        .unwrap();

    assert!(!account.remove_tag("missing"));
}

#[test]
fn has_tag_is_case_insensitive() {
    let account = AccountBuilder::new()
        .id(UserId(9))
        .owner("Ivan".to_string())
        .email("ivan@example.com".to_string())
        .add_tag("TeamA".to_string())
        .build()
        .unwrap();

    assert!(account.has_tag("teama"));
    assert!(account.has_tag("TEAMa"));
}

#[test]
fn summary_contains_key_fields() {
    let account = AccountBuilder::new()
        .id(UserId(10))
        .owner("Julia".to_string())
        .email("julia@example.com".to_string())
        .build()
        .unwrap();

    let summary = account.summary();
    assert!(summary.contains("#10"));
    assert!(summary.contains("Julia"));
    assert!(summary.contains("active"));
}

#[test]
fn complex_state_chain_behavior() {
    let mut account = AccountBuilder::new()
        .id(UserId(99))
        .owner("Kira".to_string())
        .email("kira@example.com".to_string())
        .tier(AccountTier::Enterprise)
        .add_tag("core".to_string())
        .build()
        .unwrap();

    account.record_login("10.1.1.1".to_string()).unwrap();
    account.deactivate();
    assert_eq!(
        account.record_login("10.1.1.2".to_string()),
        Err(AccountActionError::Inactive)
    );
    account.reactivate();
    account.record_login("10.1.1.3".to_string()).unwrap();
    account
        .change_email("kira2@example.com".to_string())
        .unwrap();
    account.add_tag("ops".to_string()).unwrap();

    assert_eq!(account.login_count(), 2);
    assert_eq!(account.last_login_ip(), Some("10.1.1.3"));
    assert_eq!(account.email(), "kira2@example.com");
    assert!(account.has_tag("core"));
    assert!(account.has_tag("ops"));
    assert_eq!(account.tier(), AccountTier::Enterprise);
}

// ===== Helper behavior coverage through public APIs =====

#[test]
fn normalize_token_for_roles_trims_lowercases_and_rejects_blank() {
    let mut user = build_user("mail@example.com".to_string(), "alice".to_string());

    user.grant_role(" \tAdmin\n ".to_string());
    user.grant_role("ADMIN".to_string());
    user.grant_role("  \n\t  ".to_string());

    assert_eq!(user.roles, vec!["admin", "user"]);
}

#[test]
fn normalize_token_for_login_ip_trims_and_lowercases() {
    let mut account = AccountBuilder::new()
        .id(UserId(11))
        .owner("Liam".to_string())
        .email("liam@example.com".to_string())
        .build()
        .unwrap();

    account.record_login("  EXAMPLE-IP  ".to_string()).unwrap();

    assert_eq!(account.last_login_ip(), Some("example-ip"));
}

#[test]
fn normalize_token_for_remove_tag_ignores_whitespace_and_case() {
    let mut account = AccountBuilder::new()
        .id(UserId(12))
        .owner("Maya".to_string())
        .email("maya@example.com".to_string())
        .add_tag("TeamA".to_string())
        .build()
        .unwrap();

    assert!(account.remove_tag("  teama  "));
    assert!(!account.has_tag("TEAMa"));
}

#[test]
fn is_valid_email_rejects_multiple_at_symbols_and_missing_domain_dot() {
    let multiple_at = AccountBuilder::new()
        .id(UserId(13))
        .owner("Nora".to_string())
        .email("nora@@example.com".to_string())
        .build();
    assert_eq!(multiple_at, Err(AccountBuildError::InvalidEmail));

    let missing_dot = AccountBuilder::new()
        .id(UserId(14))
        .owner("Omar".to_string())
        .email("omar@example".to_string())
        .build();
    assert_eq!(missing_dot, Err(AccountBuildError::InvalidEmail));
}

#[test]
fn is_valid_email_rejects_dot_boundary_domains_and_accepts_subdomain() {
    let starts_with_dot = AccountBuilder::new()
        .id(UserId(15))
        .owner("Pia".to_string())
        .email("pia@.example.com".to_string())
        .build();
    assert_eq!(starts_with_dot, Err(AccountBuildError::InvalidEmail));

    let ends_with_dot = AccountBuilder::new()
        .id(UserId(16))
        .owner("Quinn".to_string())
        .email("quinn@example.com.".to_string())
        .build();
    assert_eq!(ends_with_dot, Err(AccountBuildError::InvalidEmail));

    let valid_subdomain = AccountBuilder::new()
        .id(UserId(17))
        .owner("Ria".to_string())
        .email("ria@sub.example.com".to_string())
        .build();
    assert!(valid_subdomain.is_ok());
}
