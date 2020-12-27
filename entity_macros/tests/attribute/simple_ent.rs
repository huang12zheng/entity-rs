use entity::{simple_ent, Database, Ent, Id};

#[test]
fn adds_derive_clone_ent_when_missing() {
    #[simple_ent]
    struct SimpleEnt {
        #[ent(id)]
        id: Id,

        #[ent(database)]
        database: Option<Box<dyn Database>>,

        #[ent(created)]
        created: u64,

        #[ent(last_updated)]
        last_updated: u64,
    }

    let ent = SimpleEnt {
        id: 123,
        database: None,
        created: 456,
        last_updated: 789,
    };

    assert_eq!(ent.id, 123);
    assert!(ent.database.is_none());
    assert_eq!(ent.created, 456);
    assert_eq!(ent.last_updated, 789);
}

#[test]
fn fills_in_derive_clone_when_missing() {
    #[simple_ent]
    #[derive(entity::Ent)]
    struct SimpleEnt {
        #[ent(id)]
        id: Id,

        #[ent(database)]
        database: Option<Box<dyn Database>>,

        #[ent(created)]
        created: u64,

        #[ent(last_updated)]
        last_updated: u64,
    }

    let ent = SimpleEnt {
        id: 123,
        database: None,
        created: 456,
        last_updated: 789,
    };

    assert_eq!(ent.id, 123);
    assert!(ent.database.is_none());
    assert_eq!(ent.created, 456);
    assert_eq!(ent.last_updated, 789);
}

#[test]
fn fills_in_derive_ent_when_missing() {
    #[simple_ent]
    #[derive(std::clone::Clone)]
    struct SimpleEnt {
        #[ent(id)]
        id: Id,

        #[ent(database)]
        database: Option<Box<dyn Database>>,

        #[ent(created)]
        created: u64,

        #[ent(last_updated)]
        last_updated: u64,
    }

    let ent = SimpleEnt {
        id: 123,
        database: None,
        created: 456,
        last_updated: 789,
    };

    assert_eq!(ent.id, 123);
    assert!(ent.database.is_none());
    assert_eq!(ent.created, 456);
    assert_eq!(ent.last_updated, 789);
}

#[test]
fn fills_in_ent_created_field_when_missing() {
    #[simple_ent]
    #[derive(Clone, Ent)]
    struct SimpleEnt {
        #[ent(id)]
        id: Id,

        #[ent(database)]
        database: Option<Box<dyn Database>>,

        #[ent(last_updated)]
        last_updated: u64,
    }

    let ent = SimpleEnt {
        id: 123,
        database: None,
        created: 456,
        last_updated: 789,
    };

    assert_eq!(ent.id, 123);
    assert!(ent.database.is_none());
    assert_eq!(ent.created, 456);
    assert_eq!(ent.last_updated, 789);
}

#[test]
fn fills_in_ent_database_field_when_missing() {
    #[simple_ent]
    #[derive(Clone, Ent)]
    struct SimpleEnt {
        #[ent(id)]
        id: Id,

        #[ent(created)]
        created: u64,

        #[ent(last_updated)]
        last_updated: u64,
    }

    let ent = SimpleEnt {
        id: 123,
        database: None,
        created: 456,
        last_updated: 789,
    };

    assert_eq!(ent.id, 123);
    assert!(ent.database.is_none());
    assert_eq!(ent.created, 456);
    assert_eq!(ent.last_updated, 789);
}

#[test]
fn fills_in_ent_id_field_when_missing() {
    #[simple_ent]
    #[derive(Clone, Ent)]
    struct SimpleEnt {
        #[ent(database)]
        database: Option<Box<dyn Database>>,

        #[ent(created)]
        created: u64,

        #[ent(last_updated)]
        last_updated: u64,
    }

    let ent = SimpleEnt {
        id: 123,
        database: None,
        created: 456,
        last_updated: 789,
    };

    assert_eq!(ent.id, 123);
    assert!(ent.database.is_none());
    assert_eq!(ent.created, 456);
    assert_eq!(ent.last_updated, 789);
}

#[test]
fn fills_in_ent_last_updated_field_when_missing() {
    #[simple_ent]
    #[derive(Clone, Ent)]
    struct SimpleEnt {
        #[ent(id)]
        id: Id,

        #[ent(database)]
        database: Option<Box<dyn Database>>,

        #[ent(created)]
        created: u64,
    }

    let ent = SimpleEnt {
        id: 123,
        database: None,
        created: 456,
        last_updated: 789,
    };

    assert_eq!(ent.id, 123);
    assert!(ent.database.is_none());
    assert_eq!(ent.created, 456);
    assert_eq!(ent.last_updated, 789);
}

#[test]
fn fills_in_everything_missing() {
    #[simple_ent]
    struct SimpleEnt {}

    let ent = SimpleEnt {
        id: 123,
        database: None,
        created: 456,
        last_updated: 789,
    };

    assert_eq!(ent.id, 123);
    assert!(ent.database.is_none());
    assert_eq!(ent.created, 456);
    assert_eq!(ent.last_updated, 789);
}

#[test]
fn supports_renaming_ent_fields() {
    #[simple_ent(
        id = "my_id",
        database = "my_database",
        created = "my_created",
        last_updated = "my_last_updated"
    )]
    struct SimpleEnt {}

    let ent = SimpleEnt {
        my_id: 123,
        my_database: None,
        my_created: 456,
        my_last_updated: 789,
    };

    assert_eq!(ent.my_id, 123);
    assert!(ent.my_database.is_none());
    assert_eq!(ent.my_created, 456);
    assert_eq!(ent.my_last_updated, 789);
}
