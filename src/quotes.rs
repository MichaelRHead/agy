// External quotes and constants embedded into the binary at compile time via `include!("quotes.rs")`.

/// Month table offsets used by Sakamoto's day-of-week algorithm.
pub const SAKAMOTO_MONTH_OFFSETS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

/// Seconds per civil day.
pub const SECS_PER_DAY: u64 = 86_400;

/// Civil calendar epoch offset (days between 0000-03-01 and 1970-01-01).
pub const CIVIL_EPOCH_OFFSET_DAYS: i64 = 719_468;

/// SplitMix64 mixing constants for zero-dependency random selection.
pub const SPLITMIX_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
pub const SPLITMIX_MUL1: u64 = 0xBF58_476D_1CE4_E5B9;
pub const SPLITMIX_MUL2: u64 = 0x94D0_49BB_1331_11EB;

/// Lookup table of (year, December start day) for the first full day of Hanukkah (25 Kislev).
pub const HANUKKAH_START_DAYS: &[(i32, u32)] = &[
    (2024, 26),
    (2025, 15),
    (2026, 5),
    (2027, 25),
    (2028, 13),
    (2029, 2),
    (2030, 21),
    (2031, 10),
    (2032, 1), // Begins late Nov, celebrated into early December
    (2033, 17),
    (2034, 7),
    (2035, 27),
];

/// Fallback December start day for Hanukkah when a year is outside `HANUKKAH_START_DAYS`.
pub const DEFAULT_HANUKKAH_START_DAY: u32 = 15;

/// Specialized greeting templates for each month's holiday (`{name}` is replaced with the recipient).
pub const JAN_GREETINGS: &[&str] = &[
    "Happy New Year, {name}!",
    "Cheers to a bright and joyful New Year, {name}!",
    "Out with the old, in with the new — Happy New Year, {name}!",
    "Raise a glass to fresh beginnings, {name}!",
    "Wishing you 365 days of adventure and success, {name}!",
    "Sparkles, confetti, and New Year wishes to you, {name}!",
];

pub const FEB_GREETINGS: &[&str] = &[
    "Happy Valentine's Day, {name}!",
    "Sending you sweet Valentine's wishes and chocolates, {name}!",
    "Roses are red, violets are blue — Happy Valentine's Day, {name}!",
    "You're sweeter than a box of truffles, {name}!",
    "Spreading love, kindness, and Valentine cheer to you, {name}!",
    "Cupid called — have a wonderful Valentine's Day, {name}!",
];

pub const MAR_GREETINGS: &[&str] = &[
    "Happy St. Patrick's Day, {name}!",
    "May the luck of the Irish be with you today, {name}!",
    "Top o' the morning and Happy St. Paddy's Day, {name}!",
    "Wishing you four-leaf clovers and a pot of gold, {name}!",
    "Sláinte and shamrock cheers to you, {name}!",
    "May your pockets be heavy and your heart be light, {name}!",
];

pub const APR_GREETINGS: &[&str] = &[
    "Happy Earth Day, {name}!",
    "Keep calm and love our planet — Happy Earth Day, {name}!",
    "Wishing you blooming spring skies and green trails, {name}!",
    "Let's nurture nature together this Earth Day, {name}!",
    "Every day is Earth Day when you're around, {name}!",
    "Plant a seed and watch the spring blossoms grow, {name}!",
];

pub const MAY_GREETINGS: &[&str] = &[
    "Happy Memorial Day, {name}!",
    "Honoring heroes and welcoming warm sunshine, {name}!",
    "Wishing you a peaceful and reflective Memorial Day, {name}!",
    "Fire up the grill and enjoy the Memorial Day breeze, {name}!",
    "Here's to remembrance, gratitude, and bright May skies, {name}!",
    "Saluting those who served — have a meaningful Memorial Day, {name}!",
];

pub const JUN_GREETINGS: &[&str] = &[
    "Happy Juneteenth, {name}!",
    "Celebrating freedom, community, and joy with you, {name}!",
    "Wishing you a vibrant and inspiring Juneteenth, {name}!",
    "Honor the past and celebrate the future — Happy Juneteenth, {name}!",
    "Lift every voice — warm Juneteenth greetings to you, {name}!",
    "Sunshine, Jubilee, and Juneteenth blessings to you, {name}!",
];

pub const JUL_GREETINGS: &[&str] = &[
    "Happy 4th of July, {name}!",
    "Sparklers, fireworks, and summer fun to you, {name}!",
    "Have a star-spangled Independence Day, {name}!",
    "Cue the fireworks — Happy Fourth of July, {name}!",
    "Wishing you backyard barbecues and dazzling night skies, {name}!",
    "Red, white, and booming — enjoy the 4th of July, {name}!",
];

pub const AUG_GREETINGS: &[&str] = &[
    "Happy Friendship Day, {name}!",
    "Cheers to great friends and golden summer days, {name}!",
    "Life is better with friends like you — Happy Friendship Day, {name}!",
    "Sending a high-five and warm August sunshine to you, {name}!",
    "Here's to laughter, loyalty, and lifelong friendship, {name}!",
    "Thankful for your friendship today and every day, {name}!",
];

pub const SEP_GREETINGS: &[&str] = &[
    "Happy Labor Day, {name}!",
    "Kick back and relax — you've earned this Labor Day break, {name}!",
    "Cheers to hard work and a well-deserved rest, {name}!",
    "Wishing you crisp autumn breezes and a restful Labor Day, {name}!",
    "Put your feet up and enjoy the holiday, {name}!",
    "Here's to the makers, builders, and dreamers — Happy Labor Day, {name}!",
];

pub const OCT_GREETINGS: &[&str] = &[
    "Happy Halloween, {name}!",
    "Trick or treat — hope your Halloween is spooky and sweet, {name}!",
    "Boo! Wishing you a fang-tastic Halloween, {name}!",
    "Double, double, toil and trouble — Happy Halloween, {name}!",
    "Light the jack-o'-lantern and pass the candy corn, {name}!",
    "Have a gourd-geous and delightfully spooky Halloween, {name}!",
];

pub const NOV_GREETINGS: &[&str] = &[
    "Happy Thanksgiving, {name}!",
    "Gobble gobble, {name} — Happy Turkey Day!",
    "Warm autumn wishes and a thankful heart to you, {name}!",
    "Pass the stuffing and cranberry sauce, {name}!",
    "Feast mode activated — Happy Thanksgiving, {name}!",
    "Wishing you a harvest of gratitude and pumpkin pie, {name}!",
];

pub const DEC_GREETINGS: &[&str] = &[
    "Happy Hanukkah, {name}!",
    "Chag Hanukkah Sameach, {name}!",
    "Wishing you eight nights of light, warmth, and joy, {name}!",
    "Spin the dreidel and pass the crispy latkes, {name}!",
    "Happy Festival of Lights, {name}!",
    "May your menorah shine bright this Hanukkah, {name}!",
];

/// Featured holiday dishes and treats for each month.
pub const JAN_DISHES: &[&str] = &[
    "Sparkling Apple Cider Toast 🥂",
    "Southern Hoppin' John with Black-Eyed Peas & Collards 🍲",
    "Glazed New Year's Pretzel Ring 🥨",
    "Toshikoshi Soba Longevity Noodles 🍜",
    "Pomegranate & Citrus Winter Salad 🍊",
];

pub const FEB_DISHES: &[&str] = &[
    "Dark Chocolate-Covered Strawberries 🍓",
    "Warm Molten Chocolate Lava Cake 🍫",
    "Raspberry Rose Macarons 🧁",
    "Heart-Shaped Margherita Ravioli 🍝",
    "Spiced Hot Cocoa with Whipped Cream ☕",
];

pub const MAR_DISHES: &[&str] = &[
    "Traditional Irish Soda Bread with Cultured Butter 🍞",
    "Slow-Braised Corned Beef & Cabbage 🥬",
    "Savory Shepherd's Pie with Mashed Potato Crust 🥧",
    "Mint Shamrock Shake 🥤",
    "Colcannon Mashed Potatoes with Scallions 🥔",
];

pub const APR_DISHES: &[&str] = &[
    "Spring Pea & Mint Risotto 🌱",
    "Farm-Fresh Asparagus & Lemon Tart 🍋",
    "Garden Herb & Heirloom Radish Bruschetta 🥖",
    "Honey-Lavender Chamomile Iced Tea 🍵",
    "Wild Berry Rhubarb Crisp 🫐",
];

pub const MAY_DISHES: &[&str] = &[
    "Backyard Grilled Veggie & Halloumi Skewers 🍢",
    "Classic Picnic Potato Salad with Fresh Dill 🥔",
    "Sweet Corn on the Cob with Lime-Chili Butter 🌽",
    "Fresh Strawberry Shortcake with Whipped Cream 🍰",
    "Freshly Squeezed Mint Lemonade 🍋",
];

pub const JUN_DISHES: &[&str] = &[
    "Hibiscus Red Drink with Ginger & Lime 🌺",
    "Smoky Barbecue Pulled Jackfruit & Slaw Sliders 🍔",
    "Chilled Watermelon & Feta Salad with Mint 🍉",
    "Red Velvet Celebration Cupcakes 🧁",
    "Southern Peach Cobbler with Vanilla Bean Ice Cream 🍑",
];

pub const JUL_DISHES: &[&str] = &[
    "Red, White & Blue Berry Pavlova 🫐🍓",
    "Char-Grilled All-American Burgers & Caramelized Onions 🍔",
    "Firecracker Grilled Corn Salad 🌽",
    "Homemade Old-Fashioned Vanilla Ice Cream 🍨",
    "Icy Watermelon Agua Fresca 🍉",
];

pub const AUG_DISHES: &[&str] = &[
    "Heirloom Tomato, Peach & Burrata Caprese 🍅",
    "S'mores around the Late-Summer Campfire 🔥",
    "Blackberry & Lemon Verbena Galette 🥧",
    "Grilled Zucchini & Pesto Flatbread 🍕",
    "Chilled Mango Lassi 🥭",
];

pub const SEP_DISHES: &[&str] = &[
    "Warm Apple Cider Donuts with Cinnamon Sugar 🍩",
    "Cedar-Plank Roasted Salmon with Maple Glaze 🐟",
    "Harvest Farro Salad with Roasted Squash & Apples 🍎",
    "Caramelized Onion & Gruyère Galette 🧀",
    "Spiced Pear & Ginger Crisp 🍐",
];

pub const OCT_DISHES: &[&str] = &[
    "Roasted Butternut Squash Soup with Toasted Pepitas 🎃",
    "Hand-Dipped Caramel Apples with Crushed Nuts 🍏",
    "Spiced Pumpkin Bread with Maple Glaze 🍞",
    "Mulled Apple Cider with Star Anise & Cloves ☕",
    "Monster Eye Chocolate Pretzel Bites 🍫",
];

pub const NOV_DISHES: &[&str] = &[
    "Herb-Roasted Turkey with Crispy Skin 🦃",
    "Homemade Cranberry-Orange Relish 🍒",
    "Creamy Garlic Mashed Potatoes & Rich Pan Gravy 🥔",
    "Toasted Pecan & Sourdough Sage Stuffing 🍞",
    "Brown-Butter Sweet Potato Casserole with Toasted Marshmallows 🍠",
    "Classic Spiced Pumpkin Pie with Whipped Cream 🥧",
    "Green Bean Almondine with Crispy Shallots 🫘",
    "Warm Apple Cider with Cinnamon Sticks 🍎",
];

pub const DEC_DISHES: &[&str] = &[
    "Crispy Golden Potato Latkes with Applesauce & Sour Cream 🥔",
    "Jelly-Filled Sufganiyot Dusted with Powdered Sugar 🍩",
    "Sweet Noodle Kugel with Cinnamon & Golden Raisins 🍮",
    "Slow-Braised Holiday Brisket with Roasted Root Vegetables 🥕",
    "Foil-Wrapped Chocolate Hanukkah Gelt 🪙🍫",
    "Olive Oil & Citrus Cake with Pistachios 🍊",
];

/// All monthly dish slices indexed 0..12 (January..December).
pub const MONTHLY_DISHES: &[&[&str]; 12] = &[
    JAN_DISHES,
    FEB_DISHES,
    MAR_DISHES,
    APR_DISHES,
    MAY_DISHES,
    JUN_DISHES,
    JUL_DISHES,
    AUG_DISHES,
    SEP_DISHES,
    OCT_DISHES,
    NOV_DISHES,
    DEC_DISHES,
];

/// Static holiday constants for each month (excluding year-dependent start days).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HolidayStatic {
    pub month_name: &'static str,
    pub holiday_name: &'static str,
    pub greeting_prefix: &'static str,
    pub greetings: &'static [&'static str],
    pub emoji: &'static str,
    pub vibe: &'static str,
    pub fixed_start_day: Option<u32>,
    pub duration_days: u32,
    pub celebration_banner: &'static str,
    pub post_holiday_note: &'static str,
}

pub const MONTHLY_HOLIDAY_DATA: [HolidayStatic; 12] = [
    HolidayStatic {
        month_name: "January",
        holiday_name: "New Year's Day",
        greeting_prefix: "Happy New Year",
        greetings: JAN_GREETINGS,
        emoji: "🎆🥂",
        vibe: "Fresh starts and sparkling resolutions!",
        fixed_start_day: Some(1),
        duration_days: 1,
        celebration_banner: "🎆🥂 HAPPY NEW YEAR! Wishing you a joyful year ahead! 🥂🎆",
        post_holiday_note: "The new year is underway — keep those resolutions going strong!",
    },
    HolidayStatic {
        month_name: "February",
        holiday_name: "Valentine's Day",
        greeting_prefix: "Happy Valentine's Day",
        greetings: FEB_GREETINGS,
        emoji: "💘🍫",
        vibe: "Love, friendship, and sweet treats in the air!",
        fixed_start_day: Some(14),
        duration_days: 1,
        celebration_banner: "💘🌹 HAPPY VALENTINE'S DAY! Spread the love and share some chocolates! 🌹💘",
        post_holiday_note: "Discounted chocolate boxes are now in season!",
    },
    HolidayStatic {
        month_name: "March",
        holiday_name: "St. Patrick's Day",
        greeting_prefix: "Happy St. Patrick's Day",
        greetings: MAR_GREETINGS,
        emoji: "☘️🌈",
        vibe: "Shamrocks, emerald green, and a bit of luck!",
        fixed_start_day: Some(17),
        duration_days: 1,
        celebration_banner: "☘️🌈 HAPPY ST. PATRICK'S DAY! May the luck of the Irish be with you! 🌈☘️",
        post_holiday_note: "Spring is right around the corner after St. Paddy's!",
    },
    HolidayStatic {
        month_name: "April",
        holiday_name: "Earth Day",
        greeting_prefix: "Happy Earth Day",
        greetings: APR_GREETINGS,
        emoji: "🌍🌱",
        vibe: "Spring blossoms and appreciation for our planet!",
        fixed_start_day: Some(22),
        duration_days: 1,
        celebration_banner: "🌍🌱 HAPPY EARTH DAY! Plant a tree and enjoy the great outdoors! 🌱🌍",
        post_holiday_note: "Every day is a great day to care for the planet!",
    },
    HolidayStatic {
        month_name: "May",
        holiday_name: "Memorial Day",
        greeting_prefix: "Happy Memorial Day",
        greetings: MAY_GREETINGS,
        emoji: "🌺☀️",
        vibe: "Late spring sunshine, remembrance, and backyard cookouts!",
        fixed_start_day: None,
        duration_days: 1,
        celebration_banner: "🌺☀️ HAPPY MEMORIAL DAY! Remembering heroes and welcoming summer! ☀️🌺",
        post_holiday_note: "Unofficial summer season has officially begun!",
    },
    HolidayStatic {
        month_name: "June",
        holiday_name: "Juneteenth",
        greeting_prefix: "Happy Juneteenth",
        greetings: JUN_GREETINGS,
        emoji: "🌻🎉",
        vibe: "Long sunny days, freedom celebrations, and community gatherings!",
        fixed_start_day: Some(19),
        duration_days: 1,
        celebration_banner: "🌻🎉 HAPPY JUNETEENTH! Celebrating freedom, heritage, and community! 🎉🌻",
        post_holiday_note: "Summer solstice sunshine is in full swing!",
    },
    HolidayStatic {
        month_name: "July",
        holiday_name: "Independence Day",
        greeting_prefix: "Happy 4th of July",
        greetings: JUL_GREETINGS,
        emoji: "🎇🗽",
        vibe: "Summer fireworks, parades, and starlit picnics!",
        fixed_start_day: Some(4),
        duration_days: 1,
        celebration_banner: "🎇🗽 HAPPY 4TH OF JULY! Enjoy the fireworks and summer barbecue! 🗽🎇",
        post_holiday_note: "Mid-summer warmth continues after the fireworks!",
    },
    HolidayStatic {
        month_name: "August",
        holiday_name: "Friendship Day",
        greeting_prefix: "Happy Friendship Day",
        greetings: AUG_GREETINGS,
        emoji: "🏖️🤝",
        vibe: "Golden late-summer days and celebrating good friends!",
        fixed_start_day: None,
        duration_days: 1,
        celebration_banner: "🏖️🤝 HAPPY FRIENDSHIP DAY! Reach out to a great friend today! 🤝🏖️",
        post_holiday_note: "Savoring the final golden weeks of summer!",
    },
    HolidayStatic {
        month_name: "September",
        holiday_name: "Labor Day",
        greeting_prefix: "Happy Labor Day",
        greetings: SEP_GREETINGS,
        emoji: "🍎🛠️",
        vibe: "Crisp early-autumn breezes and celebrating hard work!",
        fixed_start_day: None,
        duration_days: 1,
        celebration_banner: "🍎🛠️ HAPPY LABOR DAY! Take a well-deserved break and relax! 🛠️🍎",
        post_holiday_note: "Sweater weather and apple orchard season are here!",
    },
    HolidayStatic {
        month_name: "October",
        holiday_name: "Halloween",
        greeting_prefix: "Happy Halloween",
        greetings: OCT_GREETINGS,
        emoji: "🎃👻",
        vibe: "Spooky lanterns, rustling leaves, and candy corn!",
        fixed_start_day: Some(31),
        duration_days: 1,
        celebration_banner: "🎃👻 HAPPY HALLOWEEN! Trick or treat — watch out for ghosts! 👻🎃",
        post_holiday_note: "Enjoy the leftover Halloween candy stash!",
    },
    HolidayStatic {
        month_name: "November",
        holiday_name: "Thanksgiving",
        greeting_prefix: "Happy Thanksgiving",
        greetings: NOV_GREETINGS,
        emoji: "🍁🍂",
        vibe: "Cozy autumn vibes detected.",
        fixed_start_day: None,
        duration_days: 1,
        celebration_banner: "🦃🥧 HAPPY THANKSGIVING! Pass the gravy and enjoy the feast! 🥧🦃",
        post_holiday_note: "prime time for turkey leftover sandwiches!",
    },
    HolidayStatic {
        month_name: "December",
        holiday_name: "Hanukkah",
        greeting_prefix: "Happy Hanukkah",
        greetings: DEC_GREETINGS,
        emoji: "🕎✨",
        vibe: "Warm candlelight, spinning dreidels, and the Festival of Lights!",
        fixed_start_day: None,
        duration_days: 8,
        celebration_banner: "🕎✨ CHAG HANUKKAH SAMEACH! Light the menorah and spin the dreidel! ✨🕎",
        post_holiday_note: "The warmth of the Festival of Lights carries through winter!",
    },
];
