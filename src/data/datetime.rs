use crate::array_consts;

array_consts![
    pub const OFFSET: [&str; _] = [
        "-12", "-11", "-10", "-8", "-7", "-7", "-8", "-7", "-6", "-6", "-6", "-5", "-5", "-6",
        "-5", "-4", "-4", "-4.5", "-4", "-3", "-4", "-4", "-4", "-2.5", "-3", "-3", "-3", "-3",
        "-3", "-3", "-2", "-1", "0", "-1", "1", "0", "0", "1", "1", "0", "2", "2", "2", "2", "1",
        "1", "3", "3", "2", "3", "3", "2", "3", "3", "3", "2", "3", "3", "3", "3", "3", "3", "4",
        "4.5", "4", "5", "4", "4", "4", "4.5", "5", "5", "5", "5.5", "5.5", "5.75", "6", "6",
        "6.5", "7", "7", "8", "8", "8", "8", "8", "8", "9", "9", "9", "9.5", "9.5", "10", "10",
        "10", "10", "10", "11", "11", "12", "12", "12", "12", "13", "13", "13",
    ];

    pub const ABR: [&str; _] = [
        "DST", "U", "HST", "AKDT", "PDT", "PDT", "PST", "UMST", "MDT", "MDT", "CAST", "CDT", "CDT",
        "CCST", "SPST", "EDT", "UEDT", "VST", "PYT", "ADT", "CBST", "SWST", "PSST", "NDT", "ESAST",
        "AST", "SEST", "GDT", "MST", "BST", "U", "MDT", "ADT", "CVST", "MDT", "UTC", "GMT", "BST",
        "GDT", "GST", "WEDT", "CEDT", "RDT", "CEDT", "WCAST", "NST", "GDT", "MEDT", "EST", "SDT",
        "EEDT", "SAST", "FDT", "TDT", "JDT", "LST", "JST", "AST", "KST", "AST", "EAST", "MSK",
        "SAMT", "IDT", "AST", "ADT", "MST", "GST", "CST", "AST", "WAST", "YEKT", "PKT", "IST",
        "SLST", "NST", "CAST", "BST", "MST", "SAST", "NCAST", "CST", "NAST", "MPST", "WAST", "TST",
        "UST", "NAEST", "JST", "KST", "CAST", "ACST", "EAST", "AEST", "WPST", "TST", "YST", "CPST",
        "VST", "NZST", "U", "FST", "MST", "KDT", "TST", "SST",
    ];

    pub const TEXT: [&str; _] = [
        "Dateline Standard Time",
        "UTC-11",
        "Hawaiian Standard Time",
        "Alaskan Standard Time",
        "Pacific Standard Time (Mexico)",
        "Pacific Daylight Time",
        "Pacific Standard Time",
        "US Mountain Standard Time",
        "Mountain Standard Time (Mexico)",
        "Mountain Standard Time",
        "Central America Standard Time",
        "Central Standard Time",
        "Central Standard Time (Mexico)",
        "Canada Central Standard Time",
        "SA Pacific Standard Time",
        "Eastern Standard Time",
        "US Eastern Standard Time",
        "Venezuela Standard Time",
        "Paraguay Standard Time",
        "Atlantic Standard Time",
        "Central Brazilian Standard Time",
        "SA Western Standard Time",
        "Pacific SA Standard Time",
        "Newfoundland Standard Time",
        "E. South America Standard Time",
        "Argentina Standard Time",
        "SA Eastern Standard Time",
        "Greenland Standard Time",
        "Montevideo Standard Time",
        "Bahia Standard Time",
        "UTC-02",
        "Mid-Atlantic Standard Time",
        "Azores Standard Time",
        "Cape Verde Standard Time",
        "Morocco Standard Time",
        "UTC",
        "Greenwich Mean Time",
        "British Summer Time",
        "GMT Standard Time",
        "Greenwich Standard Time",
        "W. Europe Standard Time",
        "Central Europe Standard Time",
        "Romance Standard Time",
        "Central European Standard Time",
        "W. Central Africa Standard Time",
        "Namibia Standard Time",
        "GTB Standard Time",
        "Middle East Standard Time",
        "Egypt Standard Time",
        "Syria Standard Time",
        "E. Europe Standard Time",
        "South Africa Standard Time",
        "FLE Standard Time",
        "Turkey Standard Time",
        "Israel Standard Time",
        "Libya Standard Time",
        "Jordan Standard Time",
        "Arabic Standard Time",
        "Kaliningrad Standard Time",
        "Arab Standard Time",
        "E. Africa Standard Time",
        "Moscow Standard Time",
        "Samara Time",
        "Iran Standard Time",
        "Arabian Standard Time",
        "Azerbaijan Standard Time",
        "Mauritius Standard Time",
        "Georgian Standard Time",
        "Caucasus Standard Time",
        "Afghanistan Standard Time",
        "West Asia Standard Time",
        "Yekaterinburg Time",
        "Pakistan Standard Time",
        "India Standard Time",
        "Sri Lanka Standard Time",
        "Nepal Standard Time",
        "Central Asia Standard Time",
        "Bangladesh Standard Time",
        "Myanmar Standard Time",
        "SE Asia Standard Time",
        "N. Central Asia Standard Time",
        "China Standard Time",
        "North Asia Standard Time",
        "Singapore Standard Time",
        "W. Australia Standard Time",
        "Taipei Standard Time",
        "Ulaanbaatar Standard Time",
        "North Asia East Standard Time",
        "Japan Standard Time",
        "Korea Standard Time",
        "Cen. Australia Standard Time",
        "AUS Central Standard Time",
        "E. Australia Standard Time",
        "AUS Eastern Standard Time",
        "West Pacific Standard Time",
        "Tasmania Standard Time",
        "Yakutsk Standard Time",
        "Central Pacific Standard Time",
        "Vladivostok Standard Time",
        "New Zealand Standard Time",
        "UTC+12",
        "Fiji Standard Time",
        "Magadan Standard Time",
        "Kamchatka Standard Time",
        "Tonga Standard Time",
        "Samoa Standard Time",
    ];

    pub const FULL: [&str; _] = [
        "(UTC-12:00) International Date Line West",
        "(UTC-11:00) Coordinated Universal Time-11",
        "(UTC-10:00) Hawaii",
        "(UTC-09:00) Alaska",
        "(UTC-08:00) Baja California",
        "(UTC-07:00) Pacific Time (US & Canada)",
        "(UTC-08:00) Pacific Time (US & Canada)",
        "(UTC-07:00) Arizona",
        "(UTC-07:00) Chihuahua, La Paz, Mazatlan",
        "(UTC-07:00) Mountain Time (US & Canada)",
        "(UTC-06:00) Central America",
        "(UTC-06:00) Central Time (US & Canada)",
        "(UTC-06:00) Guadalajara, Mexico City, Monterrey",
        "(UTC-06:00) Saskatchewan",
        "(UTC-05:00) Bogota, Lima, Quito",
        "(UTC-05:00) Eastern Time (US & Canada)",
        "(UTC-05:00) Indiana (East)",
        "(UTC-04:30) Caracas",
        "(UTC-04:00) Asuncion",
        "(UTC-04:00) Atlantic Time (Canada)",
        "(UTC-04:00) Cuiaba",
        "(UTC-04:00) Georgetown, La Paz, Manaus, San Juan",
        "(UTC-04:00) Santiago",
        "(UTC-03:30) Newfoundland",
        "(UTC-03:00) Brasilia",
        "(UTC-03:00) Buenos Aires",
        "(UTC-03:00) Cayenne, Fortaleza",
        "(UTC-03:00) Greenland",
        "(UTC-03:00) Montevideo",
        "(UTC-03:00) Salvador",
        "(UTC-02:00) Coordinated Universal Time-02",
        "(UTC-02:00) Mid-Atlantic - Old",
        "(UTC-01:00) Azores",
        "(UTC-01:00) Cape Verde Is.",
        "(UTC) Casablanca",
        "(UTC) Coordinated Universal Time",
        "(UTC) Edinburgh, London",
        "(UTC+01:00) Edinburgh, London",
        "(UTC) Dublin, Lisbon",
        "(UTC) Monrovia, Reykjavik",
        "(UTC+01:00) Amsterdam, Berlin, Bern, Rome, Stockholm, Vienna",
        "(UTC+01:00) Belgrade, Bratislava, Budapest, Ljubljana, Prague",
        "(UTC+01:00) Brussels, Copenhagen, Madrid, Paris",
        "(UTC+01:00) Sarajevo, Skopje, Warsaw, Zagreb",
        "(UTC+01:00) West Central Africa",
        "(UTC+01:00) Windhoek",
        "(UTC+02:00) Athens, Bucharest",
        "(UTC+02:00) Beirut",
        "(UTC+02:00) Cairo",
        "(UTC+02:00) Damascus",
        "(UTC+02:00) E. Europe",
        "(UTC+02:00) Harare, Pretoria",
        "(UTC+02:00) Helsinki, Kyiv, Riga, Sofia, Tallinn, Vilnius",
        "(UTC+03:00) Istanbul",
        "(UTC+02:00) Jerusalem",
        "(UTC+02:00) Tripoli",
        "(UTC+03:00) Amman",
        "(UTC+03:00) Baghdad",
        "(UTC+03:00) Kaliningrad, Minsk",
        "(UTC+03:00) Kuwait, Riyadh",
        "(UTC+03:00) Nairobi",
        "(UTC+03:00) Moscow, St. Petersburg, Volgograd",
        "(UTC+04:00) Samara, Ulyanovsk, Saratov",
        "(UTC+03:30) Tehran",
        "(UTC+04:00) Abu Dhabi, Muscat",
        "(UTC+04:00) Baku",
        "(UTC+04:00) Port Louis",
        "(UTC+04:00) Tbilisi",
        "(UTC+04:00) Yerevan",
        "(UTC+04:30) Kabul",
        "(UTC+05:00) Ashgabat, Tashkent",
        "(UTC+05:00) Yekaterinburg",
        "(UTC+05:00) Islamabad, Karachi",
        "(UTC+05:30) Chennai, Kolkata, Mumbai, New Delhi",
        "(UTC+05:30) Sri Jayawardenepura",
        "(UTC+05:45) Kathmandu",
        "(UTC+06:00) Astana",
        "(UTC+06:00) Dhaka",
        "(UTC+06:30) Yangon (Rangoon)",
        "(UTC+07:00) Bangkok, Hanoi, Jakarta",
        "(UTC+07:00) Novosibirsk",
        "(UTC+08:00) Beijing, Chongqing, Hong Kong, Urumqi",
        "(UTC+08:00) Krasnoyarsk",
        "(UTC+08:00) Kuala Lumpur, Singapore",
        "(UTC+08:00) Perth",
        "(UTC+08:00) Taipei",
        "(UTC+08:00) Ulaanbaatar",
        "(UTC+09:00) Irkutsk",
        "(UTC+09:00) Osaka, Sapporo, Tokyo",
        "(UTC+09:00) Seoul",
        "(UTC+09:30) Adelaide",
        "(UTC+09:30) Darwin",
        "(UTC+10:00) Brisbane",
        "(UTC+10:00) Canberra, Melbourne, Sydney",
        "(UTC+10:00) Guam, Port Moresby",
        "(UTC+10:00) Hobart",
        "(UTC+10:00) Yakutsk",
        "(UTC+11:00) Solomon Is., New Caledonia",
        "(UTC+11:00) Vladivostok",
        "(UTC+12:00) Auckland, Wellington",
        "(UTC+12:00) Coordinated Universal Time+12",
        "(UTC+12:00) Fiji",
        "(UTC+12:00) Magadan",
        "(UTC+12:00) Petropavlovsk-Kamchatsky - Old",
        "(UTC+13:00) Nuku'alofa",
        "(UTC+13:00) Samoa",
    ];
];
