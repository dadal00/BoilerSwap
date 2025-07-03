pub const KEYSPACE: &str = "boiler_swap";

pub mod tables {
    pub const USERS: &str = "users";
    pub const ITEMS: &str = "items";
    pub const CDC: &str = "cdc";
}

pub mod columns {
    pub mod users {
        pub const EMAIL: &str = "email";
        pub const EMAIL_TYPE: &str = "text";

        pub const PASSWORD_HASH: &str = "password_hash";
        pub const PASSWORD_HASH_TYPE: &str = "text";

        pub const LOCKED: &str = "locked";
        pub const LOCKED_TYPE: &str = "boolean";

        pub const PRIMARY_KEY: &str = EMAIL;
        pub const TTL: &str = "126144000";
    }

    pub mod items {
        pub const ITEM_ID: &str = "item_id";
        pub const ITEM_ID_TYPE: &str = "uuid";

        pub const ITEM_TYPE: &str = "item_type";
        pub const ITEM_TYPE_TYPE: &str = "tinyint";

        pub const TITLE: &str = "title";
        pub const TITLE_TYPE: &str = "text";

        pub const CONDITION: &str = "condition";
        pub const CONDITION_TYPE: &str = "tinyint";

        pub const LOCATION: &str = "location";
        pub const LOCATION_TYPE: &str = "tinyint";

        pub const DESCRIPTION: &str = "description";
        pub const DESCRIPTION_TYPE: &str = "text";

        pub const PRIMARY_KEY: &str = ITEM_ID;
    }
}
