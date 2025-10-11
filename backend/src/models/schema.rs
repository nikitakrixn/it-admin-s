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
    }
}

diesel::joinable!(activity_log -> users (user_id));
diesel::joinable!(api_tokens -> users (user_id));
diesel::joinable!(component_history -> computers (computer_id));
diesel::joinable!(component_history -> users (changed_by));
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
diesel::joinable!(consumable_movements -> computers (computer_id));
diesel::joinable!(consumable_movements -> consumables (consumable_id));
diesel::joinable!(consumable_movements -> employees (employee_id));
diesel::joinable!(consumable_movements -> equipment (equipment_id));
diesel::joinable!(consumable_movements -> users (performed_by));
diesel::joinable!(consumables -> hardware_types (compatible_with));
diesel::joinable!(employees -> departments (department_id));
diesel::joinable!(employees -> positions (position_id));
diesel::joinable!(equipment -> employees (employee_id));
diesel::joinable!(equipment -> hardware_types (type_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(positions -> departments (department_id));
diesel::joinable!(request_attachments -> requests (request_id));
diesel::joinable!(request_attachments -> users (uploaded_by));
diesel::joinable!(request_comments -> requests (request_id));
diesel::joinable!(request_comments -> users (user_id));
diesel::joinable!(requests -> computers (computer_id));
diesel::joinable!(requests -> equipment (equipment_id));
diesel::joinable!(software_history -> computers (computer_id));
diesel::joinable!(software_history -> software_catalog (software_catalog_id));
diesel::joinable!(software_name_mappings -> software_catalog (software_catalog_id));
diesel::joinable!(users -> employees (employee_id));

diesel::allow_tables_to_appear_in_same_query!(
    activity_log,
    api_tokens,
    component_history,
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
    departments,
    employees,
    equipment,
    hardware_types,
    notifications,
    positions,
    request_attachments,
    request_comments,
    requests,
    software_catalog,
    software_history,
    software_name_mappings,
    users,
);
