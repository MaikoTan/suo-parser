/// https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md
#[derive(Debug)]
pub enum NetSyncLogType {
    /// 0x00 LogLine
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-00-0x00-logline
    LogLine = 0x00,

    /// 0x01 ChangeZone
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-01-0x01-changezone
    ChangeZone = 0x01,

    /// 0x02 ChangePrimaryPlayer
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-02-0x02-changeprimaryplayer
    ChangePrimaryPlayer = 0x02,

    /// 0x03 AddCombatant
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-03-0x03-addcombatant
    AddCombatant = 0x03,

    /// 0x04 RemoveCombatant
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-04-0x04-removecombatant
    RemoveCombatant = 0x04,

    /// 0x0B PartyList
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-11-0x0b-partylist
    PartyList = 0x0B,

    /// 0x0C PlayerStats
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-12-0x0c-playerstats
    PlayerStats = 0x0C,

    /// 0x14 NetworkStartsCasting
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-20-0x14-networkstartscasting
    NetworkStartsCasting = 0x14,

    /// 0x15 NetworkAbility
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-21-0x15-networkability
    NetworkAbility = 0x15,

    /// 0x16 NetworkAOEAbility
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-22-0x16-networkaoeability
    NetworkAOEAbility = 0x16,

    /// 0x17 NetworkCancelAbility
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-23-0x17-networkcancelability
    NetworkCancelAbility = 0x17,

    /// 0x18 NetworkDoT
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-24-0x18-networkdot
    NetworkDoT = 0x18,

    /// 0x19 NetworkDeath
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-25-0x19-networkdeath
    NetworkDeath = 0x19,

    /// 0x1A NetworkBuff
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-26-0x1a-networkbuff
    NetworkBuff = 0x1A,

    /// 0x1B NetworkTargetIcon
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-27-0x1b-networktargeticon-head-marker
    NetworkTargetIcon = 0x1B,

    /// 0x1C NetworkRaidMarker
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-28-0x1c-networkraidmarker-floor-marker
    NetworkRaidMarker = 0x1C,

    /// 0x1D NetworkTargetMarker
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-29-0x1d-networktargetmarker-player-marker
    NetworkTargetMarker = 0x1D,

    /// 0x1E NetworkBuffRemove
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-30-0x1e-networkbuffremove
    NetworkBuffRemove = 0x1E,

    /// 0x1F NetworkGauge
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-31-0x1f-networkgauge
    NetworkGauge = 0x1F,

    /// 0x20 NetworkWorld
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-32-0x20-networkworld
    NetworkWorld = 0x20,

    /// 0x21 Network6D
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-33-0x21-network6d-actor-control
    Network6D = 0x21,

    /// 0x22 NetworkNameToggle
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-34-0x22-networknametoggle
    NetworkNameToggle = 0x22,

    /// 0x23 NetworkTether
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-35-0x23-networktether
    NetworkTether = 0x23,

    /// 0x24 LimitBreak
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-36-0x24-limitbreak
    LimitBreak = 0x24,

    /// 0x25 NetworkActionSync
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-37-0x25-networkactionsync
    NetworkActionSync = 0x25,

    /// 0x26 NetworkStatusEffects
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-38-0x26-networkstatuseffects
    NetworkStatusEffects = 0x26,

    /// 0x27 NetworkUpdateHP
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-39-0x27-networkupdatehp
    NetworkUpdateHP = 0x27,

    /// 0x28 Map
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-40-0x28-map
    Map = 0x28,

    /// 0x29 SystemLogMessage
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-41-0x29-systemlogmessage
    SystemLogMessage = 0x29,

    /// 0x2A StatusList3
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-42-0x2a-statuslist3
    StatusList3 = 0x2A,

    /// 0xF9 ParserInfo
    ParserInfo = 0xF9,

    /// 0xFA ProcessInfo
    ProcessInfo = 0xFA,

    /// 0xFB Debug
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-251-0xfb-debug
    Debug = 0xFB,

    /// 0xFC PacketDump
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-252-0xfc-packetdump
    PacketDump = 0xFC,

    /// 0xFD Version
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-253-0xfd-version
    Version = 0xFD,

    /// 0xFE Error
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-254-0xfe-error
    Error = 0xFE,

    /// 0x100 LineRegistration
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-256-0x100-lineregistration
    LineRegistration = 0x100,

    /// 0x101 MapEffect
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-257-0x101-mapeffect
    MapEffect = 0x101,

    /// 0x102 FateDirector
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-258-0x102-fatedirector
    FateDirector = 0x102,

    /// 0x103 CEDirector
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-259-0x103-cedirector
    CEDirector = 0x103,

    /// 0x104 InCombat
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-260-0x104-incombat
    InCombat = 0x104,

    /// 0x105 CombatantMemory
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-261-0x105-combatantmemory
    CombatantMemory = 0x105,

    /// 0x106 RSVData
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-262-0x106-rsvdata
    RSVData = 0x106,

    /// 0x107 StartsUsingExtra
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-263-0x107-startsusingextra
    StartsUsingExtra = 0x107,

    /// 0x108 AbilityExtra
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-264-0x108-abilityextra
    AbilityExtra = 0x108,

    /// 0x109 ContentFinderSettings
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-265-0x109-contentfindersettings
    ContentFinderSettings = 0x109,

    /// 0x10A NpcYell
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-266-0x10a-npcyell
    NpcYell = 0x10A,

    /// 0x10B BattleTalk2
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-267-0x10b-battletalk2
    BattleTalk2 = 0x10B,

    /// 0x10C Countdown
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-268-0x10c-countdown
    Countdown = 0x10C,

    /// 0x10D CountdownCancel
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-269-0x10d-countdowncancel
    CountdownCancel = 0x10D,

    /// 0x10E ActorMove
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-270-0x10e-actormove
    ActorMove = 0x10E,

    /// 0x10F ActorSetPos
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-271-0x10f-actorsetpos
    ActorSetPos = 0x10F,

    /// 0x110 SpawnNpcExtra
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-272-0x110-spawnnpcextra
    SpawnNpcExtra = 0x110,

    /// 0x111 ActorControlExtra
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-273-0x111-actorcontrolextra
    ActorControlExtra = 0x111,

    /// 0x112 ActorControlSelfExtra
    ///
    /// see https://github.com/OverlayPlugin/cactbot/blob/main/docs/LogGuide.md#line-274-0x112-actorcontrolselfextra
    ActorControlSelfExtra = 0x112,
}

impl NetSyncLogType {
    /// Returns the key name for the enum variant.
    pub fn key_name(&self) -> String {
        format!("{:?}", self)
    }

    /// Returns all keys as a vector of strings.
    pub fn all_keys() -> Vec<String> {
        // NOTE: There are some keys that are not directly mapped to enum variants.
        // See: https://github.com/OverlayPlugin/cactbot/blob/main/resources/netlog_defs.ts
        vec![
            "GameLog".to_string(), // Self::LogLine.key_name(),
            Self::ChangeZone.key_name(),
            "ChangePlayer".to_string(), // Self::ChangePrimaryPlayer.key_name(),
            "AddedCombatant".to_string(), // Self::AddCombatant.key_name(),
            "RemovedCombatant".to_string(), // Self::RemoveCombatant.key_name(),
            Self::PartyList.key_name(),
            Self::PlayerStats.key_name(),
            "StartsUsing".to_string(), // Self::NetworkStartsCasting.key_name(),
            "Ability".to_string(),     // Self::NetworkAbility.key_name(),
            Self::NetworkAOEAbility.key_name(),
            Self::NetworkCancelAbility.key_name(),
            Self::NetworkDoT.key_name(),
            "WasDefeated".to_string(), // Self::NetworkDeath.key_name(),
            "GainsEffect".to_string(), // Self::NetworkBuff.key_name(),
            "HeadMarker".to_string(),  // Self::NetworkTargetIcon.key_name(),
            Self::NetworkRaidMarker.key_name(),
            Self::NetworkTargetMarker.key_name(),
            "LosesEffect".to_string(), // Self::NetworkBuffRemove.key_name(),
            Self::NetworkGauge.key_name(),
            Self::NetworkWorld.key_name(),
            "ActorControl".to_string(), // Self::Network6D.key_name(),
            "NameToggle".to_string(),   // Self::NetworkNameToggle.key_name(),
            "Tether".to_string(),       // Self::NetworkTether.key_name(),
            Self::LimitBreak.key_name(),
            "NetworkEffectResult".to_string(), // Self::NetworkActionSync.key_name(),
            "StatusEffect".to_string(),        // Self::NetworkStatusEffects.key_name(),
            Self::NetworkUpdateHP.key_name(),
            Self::Map.key_name(),
            Self::SystemLogMessage.key_name(),
            Self::StatusList3.key_name(),
            Self::ParserInfo.key_name(),
            Self::ProcessInfo.key_name(),
            Self::Debug.key_name(),
            Self::PacketDump.key_name(),
            Self::Version.key_name(),
            Self::Error.key_name(),
            Self::LineRegistration.key_name(),
            Self::MapEffect.key_name(),
            Self::FateDirector.key_name(),
            Self::CEDirector.key_name(),
            Self::InCombat.key_name(),
            Self::CombatantMemory.key_name(),
            Self::RSVData.key_name(),
            Self::StartsUsingExtra.key_name(),
            Self::AbilityExtra.key_name(),
            Self::ContentFinderSettings.key_name(),
            Self::NpcYell.key_name(),
            Self::BattleTalk2.key_name(),
            Self::Countdown.key_name(),
            Self::CountdownCancel.key_name(),
            Self::ActorMove.key_name(),
            Self::ActorSetPos.key_name(),
            Self::SpawnNpcExtra.key_name(),
            Self::ActorControlExtra.key_name(),
            Self::ActorControlSelfExtra.key_name(),
        ]
    }
}
