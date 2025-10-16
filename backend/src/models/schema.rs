// @generated automatically by Diesel CLI.

diesel::table! {
    activity_log (id) {
        id -> Int4,
        user_id -> Nullable<Uuid>,
        #[max_length = 50]
        action -> Varchar,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        details -> Nullable<Jsonb>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ad_group_memberships (id) {
        id -> Int4,
        employee_id -> Int4,
        ad_group_id -> Int4,
        added_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ad_groups (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        distinguished_name -> Text,
        description -> Nullable<Text>,
        #[max_length = 50]
        group_type -> Nullable<Varchar>,
        #[max_length = 50]
        group_scope -> Nullable<Varchar>,
        member_count -> Nullable<Int4>,
        last_synced -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ad_sync_history (id) {
        id -> Int4,
        #[max_length = 50]
        sync_type -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        records_added -> Nullable<Int4>,
        records_updated -> Nullable<Int4>,
        records_removed -> Nullable<Int4>,
        error_message -> Nullable<Text>,
        sync_started_at -> Timestamp,
        sync_completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    alert_history (id) {
        id -> Int4,
        alert_rule_id -> Nullable<Int4>,
        #[max_length = 50]
        entity_type -> Nullable<Varchar>,
        entity_id -> Nullable<Int4>,
        #[max_length = 20]
        severity -> Varchar,
        message -> Text,
        details -> Nullable<Jsonb>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        acknowledged_by -> Nullable<Uuid>,
        acknowledged_at -> Nullable<Timestamp>,
        resolved_at -> Nullable<Timestamp>,
        resolution_notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    alert_rules (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        rule_type -> Varchar,
        #[max_length = 50]
        entity_type -> Nullable<Varchar>,
        #[max_length = 100]
        condition_field -> Varchar,
        #[max_length = 20]
        condition_operator -> Varchar,
        #[max_length = 255]
        condition_value -> Varchar,
        #[max_length = 20]
        severity -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        check_interval_minutes -> Nullable<Int4>,
        notification_channels -> Nullable<Jsonb>,
        notify_users -> Nullable<Jsonb>,
        last_checked_at -> Nullable<Timestamp>,
        last_triggered_at -> Nullable<Timestamp>,
        trigger_count -> Nullable<Int4>,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    alert_subscriptions (id) {
        id -> Int4,
        user_id -> Uuid,
        alert_rule_id -> Nullable<Int4>,
        #[max_length = 50]
        notification_method -> Varchar,
        is_active -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    antivirus_scan_history (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 50]
        scan_type -> Varchar,
        started_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        duration_seconds -> Nullable<Int4>,
        files_scanned -> Nullable<Int4>,
        threats_found -> Nullable<Int4>,
        threats_removed -> Nullable<Int4>,
        #[max_length = 50]
        scan_result -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    antivirus_threats (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        threat_name -> Varchar,
        #[max_length = 50]
        threat_type -> Nullable<Varchar>,
        #[max_length = 20]
        severity -> Nullable<Varchar>,
        file_path -> Nullable<Text>,
        detected_at -> Timestamp,
        #[max_length = 50]
        action_taken -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        resolved_at -> Nullable<Timestamp>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    api_tokens (id) {
        id -> Int4,
        #[max_length = 255]
        token -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        user_id -> Uuid,
        expires_at -> Nullable<Timestamp>,
        last_used_at -> Nullable<Timestamp>,
        is_active -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    backup_history (id) {
        id -> Int4,
        job_id -> Nullable<Int4>,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        #[max_length = 50]
        backup_type -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        started_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        duration_seconds -> Nullable<Int4>,
        backup_size_mb -> Nullable<Int8>,
        #[max_length = 500]
        backup_location -> Nullable<Varchar>,
        files_count -> Nullable<Int4>,
        success_rate -> Nullable<Numeric>,
        error_message -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    backup_jobs (id) {
        id -> Int4,
        policy_id -> Nullable<Int4>,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        #[max_length = 255]
        job_name -> Varchar,
        #[max_length = 50]
        backup_type -> Varchar,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        started_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        duration_seconds -> Nullable<Int4>,
        backup_size_mb -> Nullable<Int8>,
        #[max_length = 500]
        backup_location -> Nullable<Varchar>,
        error_message -> Nullable<Text>,
        next_run_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    backup_policies (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        backup_type -> Varchar,
        #[max_length = 50]
        frequency -> Varchar,
        retention_days -> Int4,
        #[max_length = 500]
        backup_location -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    certificate_renewals (id) {
        id -> Int4,
        old_certificate_id -> Nullable<Int4>,
        new_certificate_id -> Nullable<Int4>,
        renewed_at -> Timestamp,
        renewed_by -> Nullable<Uuid>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    certificates (id) {
        id -> Int4,
        #[max_length = 50]
        certificate_type -> Varchar,
        #[max_length = 500]
        subject -> Varchar,
        #[max_length = 500]
        issuer -> Nullable<Varchar>,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        #[max_length = 100]
        thumbprint -> Nullable<Varchar>,
        #[max_length = 50]
        algorithm -> Nullable<Varchar>,
        key_size -> Nullable<Int4>,
        valid_from -> Date,
        valid_to -> Date,
        computer_id -> Nullable<Int4>,
        #[max_length = 50]
        store_location -> Nullable<Varchar>,
        #[max_length = 50]
        store_name -> Nullable<Varchar>,
        has_private_key -> Nullable<Bool>,
        purpose -> Nullable<Text>,
        san_entries -> Nullable<Text>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    checklist_items (id) {
        id -> Int4,
        checklist_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        sort_order -> Nullable<Int4>,
        is_required -> Nullable<Bool>,
        is_completed -> Nullable<Bool>,
        completed_by -> Nullable<Uuid>,
        completed_at -> Nullable<Timestamp>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    checklist_template_items (id) {
        id -> Int4,
        template_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        sort_order -> Nullable<Int4>,
        is_required -> Nullable<Bool>,
        estimated_minutes -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    checklist_templates (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        category -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    checklists (id) {
        id -> Int4,
        template_id -> Nullable<Int4>,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        assigned_to -> Nullable<Int4>,
        started_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    component_history (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 50]
        component_type -> Varchar,
        component_id -> Int4,
        #[max_length = 20]
        action -> Varchar,
        old_value -> Nullable<Jsonb>,
        new_value -> Nullable<Jsonb>,
        changed_by -> Nullable<Uuid>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    computer_antivirus (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        product_name -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        version -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Varchar,
        is_enabled -> Nullable<Bool>,
        real_time_protection -> Nullable<Bool>,
        last_update -> Nullable<Timestamp>,
        #[max_length = 100]
        definitions_version -> Nullable<Varchar>,
        definitions_date -> Nullable<Date>,
        last_scan -> Nullable<Timestamp>,
        #[max_length = 50]
        last_scan_type -> Nullable<Varchar>,
        threats_detected -> Nullable<Int4>,
        threats_quarantined -> Nullable<Int4>,
        #[max_length = 50]
        license_status -> Nullable<Varchar>,
        license_expiry_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    computer_gpo_applications (id) {
        id -> Int4,
        computer_id -> Int4,
        gpo_id -> Int4,
        applied_at -> Nullable<Timestamp>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        error_message -> Nullable<Text>,
    }
}

diesel::table! {
    computer_gpus (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        memory_gb -> Nullable<Int4>,
        #[max_length = 50]
        memory_type -> Nullable<Varchar>,
        #[max_length = 100]
        driver_version -> Nullable<Varchar>,
        is_primary -> Nullable<Bool>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    computer_monitoring (id) {
        id -> Int4,
        computer_id -> Int4,
        cpu_usage -> Nullable<Numeric>,
        ram_usage_percent -> Nullable<Numeric>,
        ram_total_gb -> Nullable<Int4>,
        ram_used_gb -> Nullable<Numeric>,
        disk_usage -> Nullable<Jsonb>,
        cpu_temperature -> Nullable<Numeric>,
        gpu_temperature -> Nullable<Numeric>,
        uptime_seconds -> Nullable<Int8>,
        network_stats -> Nullable<Jsonb>,
        processes_count -> Nullable<Int4>,
        logged_users -> Nullable<Jsonb>,
        collected_at -> Timestamp,
    }
}

diesel::table! {
    computer_monitors (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        size_inches -> Nullable<Numeric>,
        #[max_length = 50]
        resolution -> Nullable<Varchar>,
        #[max_length = 50]
        panel_type -> Nullable<Varchar>,
        refresh_rate -> Nullable<Int4>,
        is_primary -> Nullable<Bool>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    computer_motherboards (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        #[max_length = 100]
        chipset -> Nullable<Varchar>,
        #[max_length = 100]
        bios_version -> Nullable<Varchar>,
        bios_date -> Nullable<Date>,
        #[max_length = 50]
        form_factor -> Nullable<Varchar>,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    computer_network_adapters (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 17]
        mac_address -> Nullable<Varchar>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        #[max_length = 50]
        adapter_type -> Nullable<Varchar>,
        #[max_length = 50]
        speed -> Nullable<Varchar>,
        is_primary -> Nullable<Bool>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    computer_peripherals (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        #[max_length = 50]
        connection_type -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        purchase_date -> Nullable<Date>,
        warranty_end_date -> Nullable<Date>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    computer_processors (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        cores -> Nullable<Int4>,
        threads -> Nullable<Int4>,
        #[max_length = 50]
        base_frequency -> Nullable<Varchar>,
        #[max_length = 50]
        max_frequency -> Nullable<Varchar>,
        #[max_length = 50]
        cache_size -> Nullable<Varchar>,
        #[max_length = 50]
        socket -> Nullable<Varchar>,
        #[max_length = 50]
        tdp -> Nullable<Varchar>,
        is_primary -> Nullable<Bool>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    computer_ram (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        capacity_gb -> Int4,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Nullable<Varchar>,
        #[max_length = 50]
        frequency -> Nullable<Varchar>,
        slot_number -> Nullable<Int4>,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    computer_software (id) {
        id -> Int4,
        computer_id -> Int4,
        software_catalog_id -> Int4,
        #[max_length = 100]
        version -> Nullable<Varchar>,
        #[max_length = 255]
        license_key -> Nullable<Varchar>,
        #[max_length = 50]
        license_type -> Nullable<Varchar>,
        license_end_date -> Nullable<Date>,
        installation_date -> Nullable<Date>,
        last_seen_date -> Nullable<Timestamp>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    computer_storage (id) {
        id -> Int4,
        computer_id -> Int4,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        capacity_gb -> Int4,
        #[max_length = 50]
        interface -> Nullable<Varchar>,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        #[max_length = 50]
        health_status -> Nullable<Varchar>,
        is_system_drive -> Nullable<Bool>,
        #[max_length = 100]
        mount_point -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 50]
        smart_status -> Nullable<Varchar>,
        temperature -> Nullable<Int4>,
        power_on_hours -> Nullable<Int4>,
        reallocated_sectors -> Nullable<Int4>,
        last_defrag_date -> Nullable<Date>,
        fragmentation_percent -> Nullable<Int4>,
        read_errors -> Nullable<Int4>,
        write_errors -> Nullable<Int4>,
        wear_level_percent -> Nullable<Int4>,
        total_bytes_written -> Nullable<Int8>,
        power_cycle_count -> Nullable<Int4>,
    }
}

diesel::table! {
    computers (id) {
        id -> Int4,
        #[max_length = 50]
        inventory_number -> Nullable<Varchar>,
        #[max_length = 100]
        hostname -> Nullable<Varchar>,
        #[max_length = 50]
        computer_type -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        #[max_length = 100]
        os -> Nullable<Varchar>,
        #[max_length = 50]
        os_version -> Nullable<Varchar>,
        #[max_length = 50]
        os_build -> Nullable<Varchar>,
        #[max_length = 20]
        os_architecture -> Nullable<Varchar>,
        domain_joined -> Nullable<Bool>,
        last_boot_time -> Nullable<Timestamp>,
        employee_id -> Nullable<Int4>,
        #[max_length = 100]
        location -> Nullable<Varchar>,
        purchase_date -> Nullable<Date>,
        warranty_end_date -> Nullable<Date>,
        #[max_length = 20]
        status -> Varchar,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_seen_online -> Nullable<Timestamp>,
        is_online -> Nullable<Bool>,
        #[max_length = 255]
        last_user_logged -> Nullable<Varchar>,
        bitlocker_enabled -> Nullable<Bool>,
        #[max_length = 50]
        antivirus_status -> Nullable<Varchar>,
        #[max_length = 50]
        windows_activation_status -> Nullable<Varchar>,
        location_id -> Nullable<Int4>,
    }
}

diesel::table! {
    consumable_movements (id) {
        id -> Int4,
        consumable_id -> Int4,
        equipment_id -> Nullable<Int4>,
        computer_id -> Nullable<Int4>,
        quantity -> Int4,
        #[max_length = 20]
        movement_type -> Varchar,
        employee_id -> Nullable<Int4>,
        performed_by -> Nullable<Uuid>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    consumables (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        #[max_length = 50]
        category -> Nullable<Varchar>,
        compatible_with -> Nullable<Int4>,
        quantity -> Int4,
        min_quantity -> Int4,
        #[max_length = 20]
        unit -> Nullable<Varchar>,
        price_per_unit -> Nullable<Numeric>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    contract_assets (id) {
        id -> Int4,
        contract_id -> Int4,
        #[max_length = 50]
        asset_type -> Varchar,
        asset_id -> Int4,
        quantity -> Nullable<Int4>,
        unit_price -> Nullable<Numeric>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    contracts (id) {
        id -> Int4,
        vendor_id -> Nullable<Int4>,
        #[max_length = 100]
        contract_number -> Varchar,
        #[max_length = 50]
        contract_type -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        start_date -> Date,
        end_date -> Nullable<Date>,
        contract_value -> Nullable<Numeric>,
        #[max_length = 3]
        currency -> Nullable<Varchar>,
        payment_terms -> Nullable<Text>,
        auto_renewal -> Nullable<Bool>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        responsible_employee_id -> Nullable<Int4>,
        #[max_length = 500]
        document_path -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    departments (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    document_categories (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        parent_id -> Nullable<Int4>,
        description -> Nullable<Text>,
        #[max_length = 50]
        icon -> Nullable<Varchar>,
        sort_order -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    document_permissions (id) {
        id -> Int4,
        document_id -> Int4,
        user_id -> Nullable<Uuid>,
        #[max_length = 50]
        role -> Nullable<Varchar>,
        #[max_length = 20]
        permission_type -> Varchar,
        granted_by -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    document_versions (id) {
        id -> Int4,
        document_id -> Int4,
        #[max_length = 20]
        version -> Varchar,
        #[max_length = 255]
        filename -> Varchar,
        #[max_length = 500]
        file_path -> Varchar,
        file_size -> Nullable<Int8>,
        change_notes -> Nullable<Text>,
        uploaded_by -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    documents (id) {
        id -> Int4,
        category_id -> Nullable<Int4>,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        document_type -> Nullable<Varchar>,
        #[max_length = 255]
        filename -> Varchar,
        #[max_length = 500]
        file_path -> Varchar,
        file_size -> Nullable<Int8>,
        #[max_length = 100]
        mime_type -> Nullable<Varchar>,
        #[max_length = 20]
        version -> Nullable<Varchar>,
        is_confidential -> Nullable<Bool>,
        #[max_length = 50]
        entity_type -> Nullable<Varchar>,
        entity_id -> Nullable<Int4>,
        uploaded_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    domain_account_history (id) {
        id -> Int4,
        account_id -> Int4,
        #[max_length = 50]
        action -> Varchar,
        #[max_length = 100]
        field_name -> Nullable<Varchar>,
        old_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        #[max_length = 255]
        changed_by -> Nullable<Varchar>,
        changed_at -> Timestamp,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    domain_accounts (id) {
        id -> Int4,
        employee_id -> Nullable<Int4>,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 50]
        account_type -> Varchar,
        distinguished_name -> Nullable<Text>,
        #[max_length = 255]
        user_principal_name -> Nullable<Varchar>,
        #[max_length = 100]
        sam_account_name -> Nullable<Varchar>,
        password_last_set -> Nullable<Timestamp>,
        password_expires_at -> Nullable<Timestamp>,
        password_never_expires -> Nullable<Bool>,
        must_change_password -> Nullable<Bool>,
        cannot_change_password -> Nullable<Bool>,
        account_expires_at -> Nullable<Timestamp>,
        is_enabled -> Nullable<Bool>,
        is_locked -> Nullable<Bool>,
        locked_at -> Nullable<Timestamp>,
        lockout_time -> Nullable<Timestamp>,
        last_logon -> Nullable<Timestamp>,
        #[max_length = 45]
        last_logon_ip -> Nullable<Varchar>,
        logon_count -> Nullable<Int4>,
        bad_password_count -> Nullable<Int4>,
        bad_password_time -> Nullable<Timestamp>,
        #[max_length = 500]
        home_directory -> Nullable<Varchar>,
        #[max_length = 500]
        profile_path -> Nullable<Varchar>,
        #[max_length = 500]
        script_path -> Nullable<Varchar>,
        description -> Nullable<Text>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 50]
        middle_name -> Nullable<Varchar>,
        position_id -> Nullable<Int4>,
        department_id -> Nullable<Int4>,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 50]
        ad_username -> Nullable<Varchar>,
        hire_date -> Nullable<Date>,
        termination_date -> Nullable<Date>,
        #[max_length = 20]
        status -> Varchar,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        location_id -> Nullable<Int4>,
        #[max_length = 500]
        photo_url -> Nullable<Varchar>,
        #[max_length = 100]
        telegram_username -> Nullable<Varchar>,
        is_vip -> Nullable<Bool>,
    }
}

diesel::table! {
    equipment (id) {
        id -> Int4,
        #[max_length = 50]
        inventory_number -> Nullable<Varchar>,
        type_id -> Nullable<Int4>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        employee_id -> Nullable<Int4>,
        #[max_length = 100]
        location -> Nullable<Varchar>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        #[max_length = 17]
        mac_address -> Nullable<Varchar>,
        purchase_date -> Nullable<Date>,
        warranty_end_date -> Nullable<Date>,
        #[max_length = 20]
        status -> Varchar,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        location_id -> Nullable<Int4>,
        last_maintenance_date -> Nullable<Date>,
        next_maintenance_date -> Nullable<Date>,
        toner_level -> Nullable<Int4>,
        page_count -> Nullable<Int4>,
        paper_jam_count -> Nullable<Int4>,
        total_pages_printed -> Nullable<Int4>,
        color_pages_printed -> Nullable<Int4>,
        mono_pages_printed -> Nullable<Int4>,
        #[max_length = 50]
        last_error_code -> Nullable<Varchar>,
        last_error_time -> Nullable<Timestamp>,
        supplies_status -> Nullable<Jsonb>,
    }
}

diesel::table! {
    firewall_rules (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        rule_name -> Varchar,
        #[max_length = 20]
        direction -> Nullable<Varchar>,
        #[max_length = 20]
        action -> Nullable<Varchar>,
        #[max_length = 50]
        protocol -> Nullable<Varchar>,
        #[max_length = 100]
        local_port -> Nullable<Varchar>,
        #[max_length = 100]
        remote_port -> Nullable<Varchar>,
        remote_address -> Nullable<Text>,
        enabled -> Nullable<Bool>,
        #[max_length = 50]
        profile -> Nullable<Varchar>,
        last_checked -> Nullable<Timestamp>,
    }
}

diesel::table! {
    group_policies (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        guid -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 50]
        gpo_status -> Nullable<Varchar>,
        created_date -> Nullable<Timestamp>,
        modified_date -> Nullable<Timestamp>,
        version -> Nullable<Int4>,
        last_synced -> Nullable<Timestamp>,
    }
}

diesel::table! {
    hardware_types (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 50]
        category -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    incident_evidence (id) {
        id -> Int4,
        incident_id -> Int4,
        #[max_length = 50]
        evidence_type -> Nullable<Varchar>,
        #[max_length = 255]
        file_name -> Nullable<Varchar>,
        #[max_length = 500]
        file_path -> Nullable<Varchar>,
        #[max_length = 128]
        file_hash -> Nullable<Varchar>,
        description -> Nullable<Text>,
        collected_at -> Nullable<Timestamp>,
        collected_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    inventory_audit_items (id) {
        id -> Int4,
        audit_id -> Int4,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        #[max_length = 100]
        expected_location -> Nullable<Varchar>,
        #[max_length = 100]
        actual_location -> Nullable<Varchar>,
        #[max_length = 50]
        expected_condition -> Nullable<Varchar>,
        #[max_length = 50]
        actual_condition -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Varchar,
        discrepancy_notes -> Nullable<Text>,
        #[max_length = 500]
        photo_url -> Nullable<Varchar>,
        checked_at -> Nullable<Timestamp>,
        checked_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    inventory_audits (id) {
        id -> Int4,
        #[max_length = 255]
        audit_name -> Varchar,
        audit_date -> Date,
        #[max_length = 50]
        audit_type -> Varchar,
        #[max_length = 100]
        location -> Nullable<Varchar>,
        department_id -> Nullable<Int4>,
        audited_by -> Uuid,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        total_items -> Nullable<Int4>,
        found_items -> Nullable<Int4>,
        missing_items -> Nullable<Int4>,
        damaged_items -> Nullable<Int4>,
        notes -> Nullable<Text>,
        started_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ip_addresses (id) {
        id -> Int4,
        network_segment_id -> Nullable<Int4>,
        #[max_length = 45]
        ip_address -> Varchar,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        #[max_length = 50]
        assigned_to_type -> Nullable<Varchar>,
        assigned_to_id -> Nullable<Int4>,
        #[max_length = 255]
        hostname -> Nullable<Varchar>,
        #[max_length = 17]
        mac_address -> Nullable<Varchar>,
        is_reserved -> Nullable<Bool>,
        reservation_reason -> Nullable<Text>,
        last_seen -> Nullable<Timestamp>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    kb_article_attachments (id) {
        id -> Int4,
        article_id -> Int4,
        #[max_length = 255]
        filename -> Varchar,
        #[max_length = 500]
        file_path -> Varchar,
        file_size -> Nullable<Int4>,
        #[max_length = 100]
        mime_type -> Nullable<Varchar>,
        uploaded_by -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    kb_article_requests (id) {
        id -> Int4,
        article_id -> Int4,
        request_id -> Int4,
        linked_by -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    kb_categories (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        parent_id -> Nullable<Int4>,
        description -> Nullable<Text>,
        #[max_length = 50]
        icon -> Nullable<Varchar>,
        sort_order -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    knowledge_base_articles (id) {
        id -> Int4,
        category_id -> Nullable<Int4>,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        summary -> Nullable<Text>,
        keywords -> Nullable<Text>,
        is_published -> Nullable<Bool>,
        view_count -> Nullable<Int4>,
        helpful_count -> Nullable<Int4>,
        not_helpful_count -> Nullable<Int4>,
        author_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    license_assignments (id) {
        id -> Int4,
        license_pool_id -> Int4,
        computer_id -> Nullable<Int4>,
        employee_id -> Nullable<Int4>,
        assigned_date -> Date,
        revoked_date -> Nullable<Date>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    license_pools (id) {
        id -> Int4,
        software_catalog_id -> Int4,
        #[max_length = 50]
        license_type -> Varchar,
        total_licenses -> Int4,
        used_licenses -> Nullable<Int4>,
        available_licenses -> Nullable<Int4>,
        #[max_length = 500]
        license_key -> Nullable<Varchar>,
        purchase_date -> Nullable<Date>,
        expiration_date -> Nullable<Date>,
        cost_per_license -> Nullable<Numeric>,
        total_cost -> Nullable<Numeric>,
        vendor_id -> Nullable<Int4>,
        contract_id -> Nullable<Int4>,
        auto_renewal -> Nullable<Bool>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        parent_id -> Nullable<Int4>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        location_type -> Varchar,
        address -> Nullable<Text>,
        floor -> Nullable<Int4>,
        #[max_length = 50]
        room_number -> Nullable<Varchar>,
        capacity -> Nullable<Int4>,
        description -> Nullable<Text>,
        is_active -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    maintenance_history (id) {
        id -> Int4,
        schedule_id -> Nullable<Int4>,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 50]
        maintenance_type -> Varchar,
        performed_by -> Nullable<Int4>,
        scheduled_date -> Nullable<Date>,
        actual_date -> Date,
        duration_minutes -> Nullable<Int4>,
        #[max_length = 20]
        status -> Varchar,
        work_performed -> Nullable<Text>,
        parts_replaced -> Nullable<Text>,
        cost -> Nullable<Numeric>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    maintenance_schedules (id) {
        id -> Int4,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        maintenance_type -> Varchar,
        #[max_length = 50]
        frequency -> Varchar,
        next_maintenance_date -> Date,
        last_maintenance_date -> Nullable<Date>,
        estimated_duration -> Nullable<Int4>,
        assigned_to -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        #[max_length = 20]
        priority -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    network_device_ports (id) {
        id -> Int4,
        device_id -> Int4,
        port_number -> Int4,
        #[max_length = 50]
        port_name -> Nullable<Varchar>,
        #[max_length = 50]
        port_type -> Nullable<Varchar>,
        #[max_length = 20]
        speed -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        vlan_id -> Nullable<Int4>,
        #[max_length = 50]
        connected_device_type -> Nullable<Varchar>,
        connected_device_id -> Nullable<Int4>,
        #[max_length = 17]
        mac_address -> Nullable<Varchar>,
        description -> Nullable<Text>,
        is_uplink -> Nullable<Bool>,
        last_activity -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    network_devices (id) {
        id -> Int4,
        #[max_length = 50]
        device_type -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        #[max_length = 17]
        mac_address -> Nullable<Varchar>,
        #[max_length = 255]
        management_url -> Nullable<Varchar>,
        #[max_length = 100]
        location -> Nullable<Varchar>,
        #[max_length = 100]
        firmware_version -> Nullable<Varchar>,
        port_count -> Nullable<Int4>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        purchase_date -> Nullable<Date>,
        warranty_end_date -> Nullable<Date>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    network_scans (id) {
        id -> Int4,
        #[max_length = 50]
        scan_type -> Varchar,
        network_segment_id -> Nullable<Int4>,
        #[max_length = 100]
        ip_range -> Nullable<Varchar>,
        devices_found -> Nullable<Int4>,
        scan_duration -> Nullable<Int4>,
        scan_results -> Nullable<Jsonb>,
        initiated_by -> Nullable<Uuid>,
        started_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        error_message -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    network_segments (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 45]
        network_address -> Varchar,
        #[max_length = 45]
        subnet_mask -> Varchar,
        #[max_length = 45]
        gateway -> Nullable<Varchar>,
        vlan_id -> Nullable<Int4>,
        dns_servers -> Nullable<Text>,
        dhcp_enabled -> Nullable<Bool>,
        #[max_length = 45]
        dhcp_range_start -> Nullable<Varchar>,
        #[max_length = 45]
        dhcp_range_end -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 100]
        location -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    notes (id) {
        id -> Int4,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        content -> Text,
        #[max_length = 20]
        note_type -> Nullable<Varchar>,
        is_pinned -> Nullable<Bool>,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int4,
        user_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        message -> Text,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        #[max_length = 50]
        entity_type -> Nullable<Varchar>,
        entity_id -> Nullable<Int4>,
        is_read -> Nullable<Bool>,
        read_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    patch_deployment_results (id) {
        id -> Int4,
        deployment_id -> Int4,
        computer_id -> Int4,
        #[max_length = 100]
        update_id -> Varchar,
        #[max_length = 500]
        update_title -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Varchar,
        started_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        error_code -> Nullable<Int4>,
        error_message -> Nullable<Text>,
        reboot_required -> Nullable<Bool>,
        rebooted -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    patch_deployments (id) {
        id -> Int4,
        #[max_length = 255]
        deployment_name -> Varchar,
        patch_schedule_id -> Nullable<Int4>,
        #[max_length = 50]
        target_type -> Varchar,
        target_ids -> Nullable<Array<Nullable<Int4>>>,
        update_ids -> Nullable<Array<Nullable<Text>>>,
        scheduled_at -> Timestamp,
        started_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        total_targets -> Nullable<Int4>,
        successful_count -> Nullable<Int4>,
        failed_count -> Nullable<Int4>,
        pending_count -> Nullable<Int4>,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    patch_group_members (id) {
        id -> Int4,
        patch_group_id -> Int4,
        computer_id -> Int4,
        added_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    patch_groups (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        auto_approve -> Nullable<Bool>,
        test_group -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    patch_schedules (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        target_type -> Varchar,
        target_id -> Nullable<Int4>,
        #[max_length = 20]
        patch_day -> Nullable<Varchar>,
        maintenance_window_start -> Nullable<Time>,
        maintenance_window_end -> Nullable<Time>,
        auto_approve_critical -> Nullable<Bool>,
        auto_approve_security -> Nullable<Bool>,
        auto_reboot -> Nullable<Bool>,
        reboot_delay_minutes -> Nullable<Int4>,
        notification_before_minutes -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    positions (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        department_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    project_assets (id) {
        id -> Int4,
        project_id -> Int4,
        #[max_length = 50]
        asset_type -> Varchar,
        asset_id -> Int4,
        purpose -> Nullable<Text>,
        added_at -> Nullable<Date>,
        removed_at -> Nullable<Date>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    project_members (id) {
        id -> Int4,
        project_id -> Int4,
        employee_id -> Int4,
        #[max_length = 100]
        role -> Nullable<Varchar>,
        joined_at -> Nullable<Date>,
        left_at -> Nullable<Date>,
        is_active -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    project_requests (id) {
        id -> Int4,
        project_id -> Int4,
        request_id -> Int4,
        added_by -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        project_type -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        #[max_length = 20]
        priority -> Nullable<Varchar>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        estimated_budget -> Nullable<Numeric>,
        actual_cost -> Nullable<Numeric>,
        progress_percent -> Nullable<Int4>,
        project_manager_id -> Nullable<Int4>,
        department_id -> Nullable<Int4>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rdp_sessions (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        session_id -> Nullable<Int4>,
        #[max_length = 255]
        client_name -> Nullable<Varchar>,
        #[max_length = 45]
        client_ip -> Nullable<Varchar>,
        logon_time -> Timestamp,
        logoff_time -> Nullable<Timestamp>,
        session_duration -> Nullable<Int4>,
        #[max_length = 100]
        disconnect_reason -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    remote_access_credentials (id) {
        id -> Int4,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        #[max_length = 50]
        access_type -> Varchar,
        #[max_length = 255]
        hostname -> Nullable<Varchar>,
        port -> Nullable<Int4>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        password_encrypted -> Nullable<Text>,
        #[max_length = 500]
        private_key_path -> Nullable<Varchar>,
        connection_string -> Nullable<Text>,
        notes -> Nullable<Text>,
        is_active -> Nullable<Bool>,
        last_used_at -> Nullable<Timestamp>,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    remote_access_sessions (id) {
        id -> Int4,
        credential_id -> Nullable<Int4>,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Int4,
        user_id -> Nullable<Uuid>,
        #[max_length = 50]
        access_type -> Varchar,
        #[max_length = 45]
        client_ip -> Nullable<Varchar>,
        session_started_at -> Timestamp,
        session_ended_at -> Nullable<Timestamp>,
        duration_seconds -> Nullable<Int4>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        #[max_length = 100]
        disconnect_reason -> Nullable<Varchar>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    request_attachments (id) {
        id -> Int4,
        request_id -> Int4,
        #[max_length = 255]
        filename -> Varchar,
        #[max_length = 500]
        file_path -> Varchar,
        file_size -> Nullable<Int4>,
        #[max_length = 100]
        mime_type -> Nullable<Varchar>,
        uploaded_by -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    request_comments (id) {
        id -> Int4,
        request_id -> Int4,
        user_id -> Uuid,
        comment -> Text,
        is_internal -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    request_history (id) {
        id -> Int4,
        request_id -> Int4,
        user_id -> Nullable<Uuid>,
        #[max_length = 50]
        field_name -> Varchar,
        old_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        #[max_length = 20]
        change_type -> Varchar,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    request_relations (id) {
        id -> Int4,
        request_id -> Int4,
        related_request_id -> Int4,
        #[max_length = 20]
        relation_type -> Varchar,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    request_tag_relations (id) {
        id -> Int4,
        request_id -> Int4,
        tag_id -> Int4,
        added_by -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    request_tags (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 7]
        color -> Nullable<Varchar>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    requests (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        requester_id -> Int4,
        assignee_id -> Nullable<Int4>,
        computer_id -> Nullable<Int4>,
        equipment_id -> Nullable<Int4>,
        #[max_length = 20]
        priority -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        #[max_length = 50]
        category -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        resolved_at -> Nullable<Timestamp>,
        closed_at -> Nullable<Timestamp>,
        first_response_at -> Nullable<Timestamp>,
        sla_deadline -> Nullable<Timestamp>,
        sla_breached -> Nullable<Bool>,
        estimated_time -> Nullable<Int4>,
        actual_time -> Nullable<Int4>,
        parent_request_id -> Nullable<Int4>,
        #[max_length = 50]
        recurring_schedule -> Nullable<Varchar>,
        knowledge_base_article_id -> Nullable<Int4>,
        satisfaction_rating -> Nullable<Int4>,
        satisfaction_comment -> Nullable<Text>,
    }
}

diesel::table! {
    scheduled_tasks (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 500]
        task_name -> Varchar,
        #[max_length = 500]
        task_path -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        last_run_time -> Nullable<Timestamp>,
        last_result -> Nullable<Int4>,
        next_run_time -> Nullable<Timestamp>,
        trigger_info -> Nullable<Text>,
        action_info -> Nullable<Text>,
        #[max_length = 255]
        run_as_user -> Nullable<Varchar>,
        is_enabled -> Nullable<Bool>,
        last_checked -> Nullable<Timestamp>,
    }
}

diesel::table! {
    security_incidents (id) {
        id -> Int4,
        #[max_length = 50]
        incident_number -> Varchar,
        #[max_length = 50]
        incident_type -> Varchar,
        #[max_length = 20]
        severity -> Varchar,
        computer_id -> Nullable<Int4>,
        employee_id -> Nullable<Int4>,
        detected_at -> Timestamp,
        #[max_length = 100]
        detected_by -> Nullable<Varchar>,
        resolved_at -> Nullable<Timestamp>,
        resolved_by -> Nullable<Uuid>,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        actions_taken -> Nullable<Text>,
        root_cause -> Nullable<Text>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        #[max_length = 20]
        impact_level -> Nullable<Varchar>,
        affected_systems -> Nullable<Text>,
        data_compromised -> Nullable<Bool>,
        reported_to_authorities -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    service_failures (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        service_name -> Varchar,
        failure_time -> Timestamp,
        error_message -> Nullable<Text>,
        auto_restarted -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    share_permissions (id) {
        id -> Int4,
        shared_folder_id -> Int4,
        #[max_length = 255]
        principal_name -> Varchar,
        #[max_length = 50]
        permission_type -> Nullable<Varchar>,
        is_allowed -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    shared_folders (id) {
        id -> Int4,
        computer_id -> Nullable<Int4>,
        #[max_length = 255]
        share_name -> Varchar,
        path -> Text,
        description -> Nullable<Text>,
        max_users -> Nullable<Int4>,
        current_users -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    software_catalog (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        manufacturer -> Nullable<Varchar>,
        #[max_length = 100]
        category -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 255]
        website -> Nullable<Varchar>,
        is_system -> Nullable<Bool>,
        requires_license -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 100]
        latest_version -> Nullable<Varchar>,
        is_deprecated -> Nullable<Bool>,
        #[max_length = 20]
        security_risk_level -> Nullable<Varchar>,
        vendor_id -> Nullable<Int4>,
    }
}

diesel::table! {
    software_history (id) {
        id -> Int4,
        computer_id -> Int4,
        software_catalog_id -> Int4,
        #[max_length = 20]
        action -> Varchar,
        #[max_length = 100]
        old_version -> Nullable<Varchar>,
        #[max_length = 100]
        new_version -> Nullable<Varchar>,
        detected_at -> Timestamp,
    }
}

diesel::table! {
    software_name_mappings (id) {
        id -> Int4,
        #[max_length = 255]
        original_name -> Varchar,
        #[max_length = 255]
        normalized_name -> Varchar,
        software_catalog_id -> Int4,
        #[max_length = 255]
        publisher -> Nullable<Varchar>,
        confidence_score -> Nullable<Numeric>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        employee_id -> Nullable<Int4>,
        #[max_length = 50]
        role -> Varchar,
        is_active -> Bool,
        last_login_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        two_factor_enabled -> Nullable<Bool>,
        #[max_length = 255]
        two_factor_secret -> Nullable<Varchar>,
        password_changed_at -> Nullable<Timestamp>,
        must_change_password -> Nullable<Bool>,
        failed_login_attempts -> Nullable<Int4>,
        locked_until -> Nullable<Timestamp>,
    }
}

diesel::table! {
    vendors (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        legal_name -> Nullable<Varchar>,
        #[max_length = 50]
        vendor_type -> Nullable<Varchar>,
        #[max_length = 20]
        inn -> Nullable<Varchar>,
        #[max_length = 20]
        kpp -> Nullable<Varchar>,
        #[max_length = 255]
        contact_person -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        website -> Nullable<Varchar>,
        address -> Nullable<Text>,
        rating -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    vpn_access (id) {
        id -> Int4,
        employee_id -> Int4,
        vpn_config_id -> Int4,
        granted_at -> Date,
        expires_at -> Nullable<Date>,
        granted_by -> Nullable<Uuid>,
        revoked_at -> Nullable<Date>,
        revoked_by -> Nullable<Uuid>,
        revoke_reason -> Nullable<Text>,
        is_active -> Nullable<Bool>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    vpn_configurations (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        server_address -> Varchar,
        #[max_length = 50]
        protocol -> Varchar,
        port -> Nullable<Int4>,
        #[max_length = 50]
        encryption -> Nullable<Varchar>,
        #[max_length = 50]
        authentication_method -> Nullable<Varchar>,
        split_tunneling -> Nullable<Bool>,
        dns_servers -> Nullable<Text>,
        is_active -> Nullable<Bool>,
        max_concurrent_connections -> Nullable<Int4>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    vpn_connections (id) {
        id -> Int4,
        employee_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        connection_name -> Nullable<Varchar>,
        #[max_length = 255]
        vpn_server -> Nullable<Varchar>,
        #[max_length = 50]
        vpn_protocol -> Nullable<Varchar>,
        #[max_length = 45]
        client_ip -> Nullable<Varchar>,
        #[max_length = 45]
        assigned_ip -> Nullable<Varchar>,
        connected_at -> Timestamp,
        disconnected_at -> Nullable<Timestamp>,
        session_duration_seconds -> Nullable<Int4>,
        bytes_sent -> Nullable<Int8>,
        bytes_received -> Nullable<Int8>,
        #[max_length = 100]
        disconnect_reason -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    windows_events (id) {
        id -> Int4,
        computer_id -> Int4,
        event_id -> Int4,
        #[max_length = 100]
        log_name -> Nullable<Varchar>,
        #[max_length = 20]
        level -> Nullable<Varchar>,
        #[max_length = 255]
        source -> Nullable<Varchar>,
        message -> Nullable<Text>,
        event_time -> Timestamp,
        collected_at -> Timestamp,
    }
}

diesel::table! {
    windows_features (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        feature_name -> Varchar,
        #[max_length = 500]
        display_name -> Nullable<Varchar>,
        #[max_length = 50]
        feature_type -> Nullable<Varchar>,
        #[max_length = 50]
        install_state -> Nullable<Varchar>,
        restart_needed -> Nullable<Bool>,
        last_checked -> Nullable<Timestamp>,
    }
}

diesel::table! {
    windows_missing_updates (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 100]
        update_id -> Varchar,
        #[max_length = 500]
        title -> Varchar,
        #[max_length = 20]
        kb_number -> Nullable<Varchar>,
        #[max_length = 20]
        severity -> Nullable<Varchar>,
        is_security_update -> Nullable<Bool>,
        release_date -> Nullable<Date>,
        detected_at -> Timestamp,
    }
}

diesel::table! {
    windows_services (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 255]
        service_name -> Varchar,
        #[max_length = 500]
        display_name -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        #[max_length = 50]
        startup_type -> Nullable<Varchar>,
        #[max_length = 255]
        account -> Nullable<Varchar>,
        is_critical -> Nullable<Bool>,
        last_checked -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    windows_updates (id) {
        id -> Int4,
        computer_id -> Int4,
        #[max_length = 100]
        update_id -> Varchar,
        #[max_length = 500]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 20]
        kb_number -> Nullable<Varchar>,
        installed_date -> Nullable<Timestamp>,
        is_security_update -> Nullable<Bool>,
        requires_reboot -> Nullable<Bool>,
        #[max_length = 50]
        update_classification -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    work_time_tracking (id) {
        id -> Int4,
        request_id -> Nullable<Int4>,
        employee_id -> Int4,
        #[max_length = 50]
        work_type -> Varchar,
        started_at -> Timestamp,
        ended_at -> Nullable<Timestamp>,
        duration_minutes -> Nullable<Int4>,
        work_description -> Text,
        is_billable -> Nullable<Bool>,
        hourly_rate -> Nullable<Numeric>,
        total_cost -> Nullable<Numeric>,
        approved_by -> Nullable<Uuid>,
        approved_at -> Nullable<Timestamp>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(activity_log -> users (user_id));
diesel::joinable!(ad_group_memberships -> ad_groups (ad_group_id));
diesel::joinable!(ad_group_memberships -> employees (employee_id));
diesel::joinable!(alert_history -> alert_rules (alert_rule_id));
diesel::joinable!(alert_history -> users (acknowledged_by));
diesel::joinable!(alert_rules -> users (created_by));
diesel::joinable!(alert_subscriptions -> alert_rules (alert_rule_id));
diesel::joinable!(alert_subscriptions -> users (user_id));
diesel::joinable!(antivirus_scan_history -> computers (computer_id));
diesel::joinable!(antivirus_threats -> computers (computer_id));
diesel::joinable!(api_tokens -> users (user_id));
diesel::joinable!(backup_history -> backup_jobs (job_id));
diesel::joinable!(backup_jobs -> backup_policies (policy_id));
diesel::joinable!(certificate_renewals -> users (renewed_by));
diesel::joinable!(certificates -> computers (computer_id));
diesel::joinable!(checklist_items -> checklists (checklist_id));
diesel::joinable!(checklist_items -> users (completed_by));
diesel::joinable!(checklist_template_items -> checklist_templates (template_id));
diesel::joinable!(checklist_templates -> users (created_by));
diesel::joinable!(checklists -> checklist_templates (template_id));
diesel::joinable!(checklists -> employees (assigned_to));
diesel::joinable!(component_history -> computers (computer_id));
diesel::joinable!(component_history -> users (changed_by));
diesel::joinable!(computer_antivirus -> computers (computer_id));
diesel::joinable!(computer_gpo_applications -> computers (computer_id));
diesel::joinable!(computer_gpo_applications -> group_policies (gpo_id));
diesel::joinable!(computer_gpus -> computers (computer_id));
diesel::joinable!(computer_monitoring -> computers (computer_id));
diesel::joinable!(computer_monitors -> computers (computer_id));
diesel::joinable!(computer_motherboards -> computers (computer_id));
diesel::joinable!(computer_network_adapters -> computers (computer_id));
diesel::joinable!(computer_peripherals -> computers (computer_id));
diesel::joinable!(computer_processors -> computers (computer_id));
diesel::joinable!(computer_ram -> computers (computer_id));
diesel::joinable!(computer_software -> computers (computer_id));
diesel::joinable!(computer_software -> software_catalog (software_catalog_id));
diesel::joinable!(computer_storage -> computers (computer_id));
diesel::joinable!(computers -> employees (employee_id));
diesel::joinable!(computers -> locations (location_id));
diesel::joinable!(consumable_movements -> computers (computer_id));
diesel::joinable!(consumable_movements -> consumables (consumable_id));
diesel::joinable!(consumable_movements -> employees (employee_id));
diesel::joinable!(consumable_movements -> equipment (equipment_id));
diesel::joinable!(consumable_movements -> users (performed_by));
diesel::joinable!(consumables -> hardware_types (compatible_with));
diesel::joinable!(contract_assets -> contracts (contract_id));
diesel::joinable!(contracts -> employees (responsible_employee_id));
diesel::joinable!(contracts -> vendors (vendor_id));
diesel::joinable!(document_permissions -> documents (document_id));
diesel::joinable!(document_versions -> documents (document_id));
diesel::joinable!(document_versions -> users (uploaded_by));
diesel::joinable!(documents -> document_categories (category_id));
diesel::joinable!(documents -> users (uploaded_by));
diesel::joinable!(domain_account_history -> domain_accounts (account_id));
diesel::joinable!(domain_accounts -> employees (employee_id));
diesel::joinable!(employees -> departments (department_id));
diesel::joinable!(employees -> locations (location_id));
diesel::joinable!(employees -> positions (position_id));
diesel::joinable!(equipment -> employees (employee_id));
diesel::joinable!(equipment -> hardware_types (type_id));
diesel::joinable!(equipment -> locations (location_id));
diesel::joinable!(firewall_rules -> computers (computer_id));
diesel::joinable!(incident_evidence -> security_incidents (incident_id));
diesel::joinable!(incident_evidence -> users (collected_by));
diesel::joinable!(inventory_audit_items -> inventory_audits (audit_id));
diesel::joinable!(inventory_audit_items -> users (checked_by));
diesel::joinable!(inventory_audits -> departments (department_id));
diesel::joinable!(inventory_audits -> users (audited_by));
diesel::joinable!(ip_addresses -> network_segments (network_segment_id));
diesel::joinable!(kb_article_attachments -> knowledge_base_articles (article_id));
diesel::joinable!(kb_article_attachments -> users (uploaded_by));
diesel::joinable!(kb_article_requests -> knowledge_base_articles (article_id));
diesel::joinable!(kb_article_requests -> requests (request_id));
diesel::joinable!(kb_article_requests -> users (linked_by));
diesel::joinable!(knowledge_base_articles -> kb_categories (category_id));
diesel::joinable!(knowledge_base_articles -> users (author_id));
diesel::joinable!(license_assignments -> computers (computer_id));
diesel::joinable!(license_assignments -> employees (employee_id));
diesel::joinable!(license_assignments -> license_pools (license_pool_id));
diesel::joinable!(license_pools -> contracts (contract_id));
diesel::joinable!(license_pools -> software_catalog (software_catalog_id));
diesel::joinable!(license_pools -> vendors (vendor_id));
diesel::joinable!(maintenance_history -> employees (performed_by));
diesel::joinable!(maintenance_history -> maintenance_schedules (schedule_id));
diesel::joinable!(maintenance_schedules -> employees (assigned_to));
diesel::joinable!(network_device_ports -> network_devices (device_id));
diesel::joinable!(network_scans -> network_segments (network_segment_id));
diesel::joinable!(network_scans -> users (initiated_by));
diesel::joinable!(notes -> users (created_by));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(patch_deployment_results -> computers (computer_id));
diesel::joinable!(patch_deployment_results -> patch_deployments (deployment_id));
diesel::joinable!(patch_deployments -> patch_schedules (patch_schedule_id));
diesel::joinable!(patch_deployments -> users (created_by));
diesel::joinable!(patch_group_members -> computers (computer_id));
diesel::joinable!(patch_group_members -> patch_groups (patch_group_id));
diesel::joinable!(positions -> departments (department_id));
diesel::joinable!(project_assets -> projects (project_id));
diesel::joinable!(project_members -> employees (employee_id));
diesel::joinable!(project_members -> projects (project_id));
diesel::joinable!(project_requests -> projects (project_id));
diesel::joinable!(project_requests -> requests (request_id));
diesel::joinable!(project_requests -> users (added_by));
diesel::joinable!(projects -> departments (department_id));
diesel::joinable!(projects -> employees (project_manager_id));
diesel::joinable!(rdp_sessions -> computers (computer_id));
diesel::joinable!(remote_access_credentials -> users (created_by));
diesel::joinable!(remote_access_sessions -> remote_access_credentials (credential_id));
diesel::joinable!(remote_access_sessions -> users (user_id));
diesel::joinable!(request_attachments -> requests (request_id));
diesel::joinable!(request_attachments -> users (uploaded_by));
diesel::joinable!(request_comments -> requests (request_id));
diesel::joinable!(request_comments -> users (user_id));
diesel::joinable!(request_history -> requests (request_id));
diesel::joinable!(request_history -> users (user_id));
diesel::joinable!(request_relations -> users (created_by));
diesel::joinable!(request_tag_relations -> request_tags (tag_id));
diesel::joinable!(request_tag_relations -> requests (request_id));
diesel::joinable!(request_tag_relations -> users (added_by));
diesel::joinable!(requests -> computers (computer_id));
diesel::joinable!(requests -> equipment (equipment_id));
diesel::joinable!(scheduled_tasks -> computers (computer_id));
diesel::joinable!(security_incidents -> computers (computer_id));
diesel::joinable!(security_incidents -> employees (employee_id));
diesel::joinable!(security_incidents -> users (resolved_by));
diesel::joinable!(service_failures -> computers (computer_id));
diesel::joinable!(share_permissions -> shared_folders (shared_folder_id));
diesel::joinable!(shared_folders -> computers (computer_id));
diesel::joinable!(software_catalog -> vendors (vendor_id));
diesel::joinable!(software_history -> computers (computer_id));
diesel::joinable!(software_history -> software_catalog (software_catalog_id));
diesel::joinable!(software_name_mappings -> software_catalog (software_catalog_id));
diesel::joinable!(users -> employees (employee_id));
diesel::joinable!(vpn_access -> employees (employee_id));
diesel::joinable!(vpn_access -> vpn_configurations (vpn_config_id));
diesel::joinable!(vpn_connections -> employees (employee_id));
diesel::joinable!(windows_events -> computers (computer_id));
diesel::joinable!(windows_features -> computers (computer_id));
diesel::joinable!(windows_missing_updates -> computers (computer_id));
diesel::joinable!(windows_services -> computers (computer_id));
diesel::joinable!(windows_updates -> computers (computer_id));
diesel::joinable!(work_time_tracking -> employees (employee_id));
diesel::joinable!(work_time_tracking -> requests (request_id));
diesel::joinable!(work_time_tracking -> users (approved_by));

diesel::allow_tables_to_appear_in_same_query!(
    activity_log,
    ad_group_memberships,
    ad_groups,
    ad_sync_history,
    alert_history,
    alert_rules,
    alert_subscriptions,
    antivirus_scan_history,
    antivirus_threats,
    api_tokens,
    backup_history,
    backup_jobs,
    backup_policies,
    certificate_renewals,
    certificates,
    checklist_items,
    checklist_template_items,
    checklist_templates,
    checklists,
    component_history,
    computer_antivirus,
    computer_gpo_applications,
    computer_gpus,
    computer_monitoring,
    computer_monitors,
    computer_motherboards,
    computer_network_adapters,
    computer_peripherals,
    computer_processors,
    computer_ram,
    computer_software,
    computer_storage,
    computers,
    consumable_movements,
    consumables,
    contract_assets,
    contracts,
    departments,
    document_categories,
    document_permissions,
    document_versions,
    documents,
    domain_account_history,
    domain_accounts,
    employees,
    equipment,
    firewall_rules,
    group_policies,
    hardware_types,
    incident_evidence,
    inventory_audit_items,
    inventory_audits,
    ip_addresses,
    kb_article_attachments,
    kb_article_requests,
    kb_categories,
    knowledge_base_articles,
    license_assignments,
    license_pools,
    locations,
    maintenance_history,
    maintenance_schedules,
    network_device_ports,
    network_devices,
    network_scans,
    network_segments,
    notes,
    notifications,
    patch_deployment_results,
    patch_deployments,
    patch_group_members,
    patch_groups,
    patch_schedules,
    positions,
    project_assets,
    project_members,
    project_requests,
    projects,
    rdp_sessions,
    remote_access_credentials,
    remote_access_sessions,
    request_attachments,
    request_comments,
    request_history,
    request_relations,
    request_tag_relations,
    request_tags,
    requests,
    scheduled_tasks,
    security_incidents,
    service_failures,
    share_permissions,
    shared_folders,
    software_catalog,
    software_history,
    software_name_mappings,
    users,
    vendors,
    vpn_access,
    vpn_configurations,
    vpn_connections,
    windows_events,
    windows_features,
    windows_missing_updates,
    windows_services,
    windows_updates,
    work_time_tracking,
);
