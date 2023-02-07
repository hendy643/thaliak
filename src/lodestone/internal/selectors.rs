pub(crate) static PROFILE_NAME_SELECTOR: &str = "div.frame__chara__box:nth-child(2) > .frame__chara__name";
pub(crate) static PROFILE_NAMEDAY_SELECTOR: &str = ".character-block__birth";
pub(crate) static PROFILE_FC_NAME_SELECTOR: &str = ".character__freecompany__name > h4:nth-child(2) > a:nth-child(1)";
pub(crate) static PROFILE_BIO_SELECTOR: &str = ".character__selfintroduction";
pub(crate) static PROFILE_GC_NAME_SELECTOR: &str = "div.character-block:nth-child(4) > div:nth-child(2) > p:nth-child(2)";
pub(crate) static PROFILE_DEITY_SELECTOR: &str = "p.character-block__name:nth-child(4)";
pub(crate) static PROFILE_TITLE_SELECTOR: &str = ".frame__chara__title";
pub(crate) static PROFILE_RACE_CLAN_GENDER_SELECTOR: &str = "div.character-block:nth-child(1) > div:nth-child(2) > p:nth-child(2)";

pub(crate) static PROFILE_GC_REGEX: &str = r#"([\w|\s]{2,})\s/\s(.+)"#;
pub(crate) static PROFILE_RACE_CLAN_GENDER_REGEX: &str = r#"(.+)<br>(.+) / (.+)"#;

pub(crate) static FC_NAME_SELECTOR: &str = ".freecompany__text__name";
pub(crate) static FC_MEMBER_COUNT_SELECTOR: &str = "p.freecompany__text:nth-of-type(6)";
pub(crate) static FC_NO_ESTATE_SELECTOR: &str = ".freecompany__estate__none";
pub(crate) static FC_ESTATE_GREETING_SELECTOR: &str = ".freecompany__estate__greeting";
pub(crate) static FC_ESTATE_NAME_SELECTOR: &str = ".freecompany__estate__name";
pub(crate) static FC_ESTATE_PLOT_SELECTOR: &str = ".freecompany__estate__text";
pub(crate) static FC_FORMED_SELECTOR: &str = "p.freecompany__text:nth-of-type(5) > script";
pub(crate) static FC_GRAND_COMPANY_SELECTOR: &str = "div.ldst__window:nth-child(1) > div:nth-child(2) > a:nth-child(1) > div:nth-child(2) > p:nth-child(1)";
pub(crate) static FC_RANK_SELECTOR: &str = "p.freecompany__text:nth-of-type(7)";
pub(crate) static FC_SERVER_SELECTOR: &str = "div.ldst__window:nth-child(1) > div:nth-child(2) > a:nth-child(1) > div:nth-child(2) > p:nth-child(3)";
pub(crate)static FC_SLOGAN_SELECTOR: &str = ".freecompany__text__message";
pub(crate) static FC_TAG_SELECTOR: &str = ".freecompany__text.freecompany__text__tag";

pub(crate) static FC_FORMED_REGEX: &str = r#".*ldst_strftime\((\d*)"#;
pub(crate) static FC_GRAND_COMPANY_REGEX: &str = r#"(Immortal Flames|The Maelstrom|The Order of the Twin Adder)"#;
pub(crate) static FC_SERVER_REGEX: &str = r#"(?P<World>\w*)\s+\[(?P<DC>\w*)\]"#;

pub(crate) static LS_NAME_SELECTOR: &str = ".heading__linkshell__name";
pub(crate) static LS_MEMBER_SELECTOR: &str = ".entry__link";
pub(crate) static LS_MEMBER_NAME_SELECTOR: &str = ".entry__name";

pub(crate) static LS_MEMBER_ID_REGEX: &str = r#"/lodestone/character/(\d+)/"#;
