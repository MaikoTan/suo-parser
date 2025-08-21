pub mod semantic_ast {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Program {
        pub defines: Vec<DefineStmt>,
        pub hide_alls: Vec<HideAllStmt>,
        pub alert_alls: Vec<AlertAllStmt>,
        pub entries: Vec<EntryStmt>,
    }

    #[derive(Debug, Clone)]
    pub enum Statement {
        Define(DefineStmt),
        Entry(EntryStmt),
        AnySync(AnySyncStmt),
        Sync(SyncStmt),
        NetSync(NetSyncStmt),
        Window(WindowStmt),
        Jump(JumpStmt),
        Duration(DurationStmt),
        HideAll(HideAllStmt),
        AlertAll(AlertAllStmt),
    }

    #[derive(Debug, Clone)]
    pub struct EntryStmt {
        pub time: Time,
        pub name: String,
        pub sync: Option<AnySyncStmt>,
        pub window: Option<WindowStmt>,
        pub duration: Option<DurationStmt>,
        pub jump: Option<JumpStmt>,
    }

    #[derive(Debug, Clone)]
    pub enum AnySyncStmt {
        SyncStmt(SyncStmt),
        NetSyncStmt(NetSyncStmt),
    }

    #[derive(Debug, Clone)]
    pub struct SyncStmt {
        pub regex: String,
    }

    #[derive(Debug, Clone)]
    pub struct NetSyncStmt {
        pub sync_type: String,
        pub fields: HashMap<String, String>,
    }

    #[derive(Debug, Clone)]
    pub struct WindowStmt {
        pub before: Time,
        pub after: Option<Time>,
    }

    #[derive(Debug, Clone)]
    pub struct DurationStmt {
        pub time: Time,
    }

    #[derive(Debug, Clone)]
    pub struct JumpStmt {
        pub time: Time,
    }

    #[derive(Debug, Clone)]
    pub struct HideAllStmt {
        pub name: String,
    }

    #[derive(Debug, Clone)]
    pub struct AlertAllStmt {
        pub name: String,
        pub before: Option<Time>,
        pub sound: Option<String>,
    }

    #[derive(Debug, Clone)]
    pub struct DefineStmt {
        pub define_type: DefineType,
        pub name: String,
        pub file: String,
    }

    /// currently only alertsound supported (?)
    ///
    /// @see https://github.com/grindingcoil/act_timeline/blob/master/doc/TimelineSyntax.md#%E8%AD%A6%E5%91%8A%E9%9F%B3%E3%81%AE%E5%88%A5%E5%90%8D%E8%A8%AD%E5%AE%9A
    /// @see https://github.com/grindingcoil/act_timeline/blob/d1c82613dfc9ef5136986cafe0fb96bf42cff3be/src/TimelineLoader.cs#L133
    #[derive(Debug, Clone)]
    pub enum DefineType {
        AlertSound,
    }

    #[derive(Debug, Clone)]
    pub enum Time {
        Integer(u64),
        Float(f64),
    }
}
