// ── Structs ──────────────────────────────────────────────────────────

pub struct UserSeed {
    pub username: &'static str,
    pub is_admin: bool,
}

pub struct TopicSeed {
    pub name: &'static str,
    pub description: &'static str,
    pub allowed_tags: &'static [&'static str],
}

pub struct PostSeed {
    pub topic: &'static str,
    pub title: &'static str,
    pub content: &'static str,
    pub creator: &'static str,
    pub tags: &'static [&'static str],
    pub image_url: Option<&'static str>,
    pub days_ago: u32,
    pub view_count: u64,
}

pub struct CommentSeed {
    pub post_title: &'static str,
    pub content: &'static str,
    pub sender: &'static str,
    pub days_ago: u32,
}

pub struct ReplySeed {
    pub post_title: &'static str,
    pub comment_idx: usize,
    pub parent_reply_idx: Option<usize>,
    pub content: &'static str,
    pub sender: &'static str,
    pub days_ago: u32,
}

pub struct VoteSeed {
    pub post_title: &'static str,
    pub voter: &'static str,
    pub direction: i16,
}

pub struct CmtVoteSeed {
    pub comment_idx: usize,
    pub voter: &'static str,
    pub direction: i16,
}

pub struct FollowSeed {
    pub follower: &'static str,
    pub board_name: &'static str,
}

pub struct PrefSeed {
    pub user: &'static str,
    pub sort: &'static str,
    pub muted_boards: &'static [&'static str],
}

pub struct FriendRequestSeed {
    pub sender: &'static str,
    pub receiver: &'static str,
    pub status: &'static str,
}

pub struct FriendshipSeed {
    pub user: &'static str,
    pub friend: &'static str,
}

pub struct ReportSeed {
    pub reporter: &'static str,
    pub target_type: &'static str,
    pub target_post_title: &'static str,
    pub target_comment_idx: Option<usize>,
    pub reason: &'static str,
    pub resolved: bool,
}

pub struct ModActionSeed {
    pub moderator: &'static str,
    pub action_type: &'static str,
    pub target_post_title: &'static str,
    pub reason: &'static str,
}

pub struct SessionSeed {
    pub username: &'static str,
}

// ── Users ────────────────────────────────────────────────────────────

pub static USERS: &[UserSeed] = &[
    UserSeed { username: "mouse", is_admin: true },
    UserSeed { username: "alice", is_admin: false },
    UserSeed { username: "bob", is_admin: false },
    UserSeed { username: "swiftowl", is_admin: false },
    UserSeed { username: "bravebadger", is_admin: false },
    UserSeed { username: "calmpanda", is_admin: false },
    UserSeed { username: "cleverfox", is_admin: false },
    UserSeed { username: "deepmole", is_admin: false },
    UserSeed { username: "eagerhare", is_admin: false },
    UserSeed { username: "fancyrat", is_admin: false },
    UserSeed { username: "gentleotter", is_admin: false },
    UserSeed { username: "happybunny", is_admin: false },
    UserSeed { username: "jollyfrog", is_admin: false },
    UserSeed { username: "keenwolf", is_admin: false },
    UserSeed { username: "lazysloth", is_admin: false },
    UserSeed { username: "milddeer", is_admin: false },
    UserSeed { username: "noblelion", is_admin: false },
    UserSeed { username: "oddotter", is_admin: false },
    UserSeed { username: "proudbear", is_admin: false },
    UserSeed { username: "quietowl", is_admin: false },
    UserSeed { username: "royalcat", is_admin: false },
    UserSeed { username: "shysnake", is_admin: false },
    UserSeed { username: "sillygoat", is_admin: false },
    UserSeed { username: "tinybat", is_admin: false },
    UserSeed { username: "wildmoose", is_admin: false },
];

// ── Sessions ─────────────────────────────────────────────────────────

pub static SESSIONS: &[SessionSeed] = &[
    SessionSeed { username: "mouse" },
    SessionSeed { username: "alice" },
    SessionSeed { username: "bob" },
    SessionSeed { username: "swiftowl" },
    SessionSeed { username: "eagerhare" },
];

// ── Topics ───────────────────────────────────────────────────────────

pub static TOPICS: &[TopicSeed] = &[
    TopicSeed {
        name: "announcements",
        description: "Site news, updates, and official announcements from the TwoMice team",
        allowed_tags: &["site-news", "update", "maintenance", "rules"],
    },
    TopicSeed {
        name: "introductions",
        description: "New to the burrow? Say hello and tell us about yourself",
        allowed_tags: &[],
    },
    TopicSeed {
        name: "general",
        description: "General discussion about anything and everything mouse-related",
        allowed_tags: &["discussion", "question", "poll", "humor", "meta"],
    },
    TopicSeed {
        name: "cheese",
        description: "The finest cheeses from around the burrow. Camembert, Gouda, and beyond!",
        allowed_tags: &[
            "cheddar",
            "gouda",
            "blue",
            "brie",
            "tasting",
            "recommendation",
            "recipe",
            "discussion",
        ],
    },
    TopicSeed {
        name: "tech",
        description: "Bits, bytes, and tiny keyboards. Hardware and software for mice, by mice",
        allowed_tags: &[
            "rust",
            "javascript",
            "hardware",
            "keyboards",
            "webdev",
            "tutorial",
            "showcase",
            "discussion",
        ],
    },
    TopicSeed {
        name: "art",
        description: "Show off your tiny paintings, squeak-ature drawings, and nest photography",
        allowed_tags: &[
            "watercolor",
            "digital",
            "photography",
            "illustration",
            "traditional",
            "technique",
            "showcase",
        ],
    },
    TopicSeed {
        name: "gaming",
        description: "From Maze Runner to Cheese Heist — all things gaming",
        allowed_tags: &[
            "speedrun",
            "review",
            "indie",
            "retro",
            "recommendation",
            "discussion",
            "screenshot",
        ],
    },
    TopicSeed {
        name: "books",
        description: "Book club for well-read rodents. Reviews, recommendations, and literary chat",
        allowed_tags: &[
            "fiction",
            "non-fiction",
            "fantasy",
            "sci-fi",
            "review",
            "recommendation",
            "discussion",
        ],
    },
    TopicSeed {
        name: "off-topic",
        description: "Anything goes — casual chatter, pets, music, and more",
        allowed_tags: &["casual", "pets", "weather", "music", "movies", "discussion"],
    },
    TopicSeed {
        name: "cooking",
        description: "Recipes, kitchen experiments, and culinary adventures for the culinarily curious mouse",
        allowed_tags: &[
            "recipe",
            "baking",
            "fermentation",
            "review",
            "vegetarian",
            "discussion",
        ],
    },
];

// ── Posts ────────────────────────────────────────────────────────────

pub static POSTS: &[PostSeed] = &[
    // ── announcements ────────────────────────────────────────────────
    PostSeed {
        topic: "announcements",
        title: "Welcome to TwoMice!",
        content: "Hey everyone! Welcome to TwoMice, the coziest corner of the internet. Grab some cheese and make yourself at home. Share your stories, ask questions, and don't forget to squeak hello!\n\nWe've been working hard to build a safe, friendly place for mice of all kinds to connect. With new boards being added regularly and a growing community, we think you'll love it here.\n\nIf you're new, head over to the introductions board and say hi!",
        creator: "mouse",
        tags: &["site-news"],
        image_url: None,
        days_ago: 45,
        view_count: 340,
    },
    PostSeed {
        topic: "announcements",
        title: "Community Guidelines",
        content: "A few ground rules to keep our burrow friendly:\n\n1. Be kind to other mice — no harassment, hate speech, or personal attacks\n2. No spam or advertising — this is a community, not a billboard\n3. Keep discussions in the right boards — helps everyone find what they need\n4. Have fun! That's the whole point\n\nThat's it. We're pretty chill here. Break the rules and a moderator might have a word with you, but we're all about second chances.",
        creator: "mouse",
        tags: &["rules"],
        image_url: None,
        days_ago: 44,
        view_count: 180,
    },
    PostSeed {
        topic: "announcements",
        title: "Site Updates — June 2025",
        content: "We just rolled out a bunch of improvements:\n\n- New boards: cooking and off-topic are now live!\n- Performance improvements across the board (pun intended)\n- Fixed the notification system — you should actually get them now\n- Added per-board tag filtering (check the tech board for details)\n\nAs always, reach out if you spot any bugs. Happy posting!",
        creator: "mouse",
        tags: &["update", "maintenance"],
        image_url: None,
        days_ago: 10,
        view_count: 95,
    },
    // ── introductions ────────────────────────────────────────────────
    PostSeed {
        topic: "introductions",
        title: "Hi everyone, I'm new here!",
        content: "Hey everyone! I've been lurking for a few weeks and finally decided to make an account. I'm a tiny mouse from a small burrow in the countryside. I love cheese (obviously), cozy video games, and reading fantasy novels.\n\nLooking forward to getting to know everyone! What's the best way to get involved around here?",
        creator: "happybunny",
        tags: &[],
        image_url: None,
        days_ago: 12,
        view_count: 60,
    },
    PostSeed {
        topic: "introductions",
        title: "Long-time lurker, finally posting",
        content: "I've been visiting TwoMice since the early days but never made an account until now. The community here seems genuinely kind and I wanted to be part of it.\n\nA bit about me: I'm into mechanical keyboards, sourdough baking, and I run a small art blog on the side. Hoping to find some fellow tech mice here!",
        creator: "shysnake",
        tags: &[],
        image_url: None,
        days_ago: 8,
        view_count: 45,
    },
    PostSeed {
        topic: "introductions",
        title: "Greetings from the UK burrow!",
        content: "Hello from across the pond! Just discovered this lovely little corner of the web and had to join. I'm a cheese enthusiast (cheddar is life), a terrible gamer, and an enthusiastic but mediocre cook.\n\nSeems like there's a nice mix of boards here — looking forward to exploring them all. Cheers!",
        creator: "jollyfrog",
        tags: &[],
        image_url: None,
        days_ago: 6,
        view_count: 38,
    },
    // ── general ──────────────────────────────────────────────────────
    PostSeed {
        topic: "general",
        title: "What's everyone snacking on today?",
        content: "Curious what my fellow mice are nibbling on right now. I just finished off a small wedge of aged gouda with some dried cranberries on the side. Perfect afternoon snack.\n\nDrop your current snack below — always looking for new things to try!",
        creator: "alice",
        tags: &["discussion"],
        image_url: None,
        days_ago: 15,
        view_count: 120,
    },
    PostSeed {
        topic: "general",
        title: "Forum rules and guidelines",
        content: "A few ground rules to keep our burrow friendly:\n\n1. Be kind to other mice\n2. No spam or advertising\n3. Keep discussions in the right boards\n4. Have fun!\n\nThat's it. We're pretty chill here.",
        creator: "alice",
        tags: &["discussion", "meta"],
        image_url: None,
        days_ago: 42,
        view_count: 200,
    },
    PostSeed {
        topic: "general",
        title: "Hot take: mornings are better than nights",
        content: "I know this is controversial, but hear me out. Mornings are objectively better. The air is fresh, everything is quiet, and that first nibble of breakfast cheese tastes better than any midnight snack ever could.\n\nNights have their charm too, sure. Cozy lamps, star gazing, the thrill of sneaking around. But mornings win for productivity and peace of mind.\n\nChange my mind!",
        creator: "eagerhare",
        tags: &["discussion", "poll"],
        image_url: None,
        days_ago: 14,
        view_count: 85,
    },
    PostSeed {
        topic: "general",
        title: "What's your favorite mouse pun?",
        content: "I'll start: 'I'm not squeaking, I'm projecting.'\n\nYes, I know it's terrible. That's the point. Hit me with your best (worst) mouse-related puns. The cringier the better.",
        creator: "jollyfrog",
        tags: &["humor"],
        image_url: None,
        days_ago: 7,
        view_count: 150,
    },
    PostSeed {
        topic: "general",
        title: "Should we have a community event?",
        content: "I was thinking it'd be fun to organize something — maybe a cheese tasting weekend where everyone tries and reviews a cheese, or a gaming night where we all play the same game and compare experiences.\n\nWhat do you all think? Any ideas for what kind of event would work best? Drop a comment with your suggestions!",
        creator: "cleverfox",
        tags: &["discussion", "poll"],
        image_url: None,
        days_ago: 5,
        view_count: 72,
    },
    // ── cheese ────────────────────────────────────────────────────────
    PostSeed {
        topic: "cheese",
        title: "Best cheddar I've ever nibbled",
        content: "Found this amazing aged cheddar at the farmer's market yesterday. Sharp, crumbly, with those perfect little crystals. 10/10 would nibble again. What's your favorite cheese discovery this month?",
        creator: "bob",
        tags: &["cheddar", "tasting"],
        image_url: Some("https://images.unsplash.com/photo-1552768801-3e4fc6ec7e78?w=800"),
        days_ago: 35,
        view_count: 210,
    },
    PostSeed {
        topic: "cheese",
        title: "Gouda vs Edam: the ultimate showdown",
        content: "I've been going back and forth between these two Dutch classics. Gouda has that rich, caramel sweetness when aged, but Edam's nutty mildness is so versatile. Which side are you on?",
        creator: "mouse",
        tags: &["gouda", "discussion"],
        image_url: Some("https://images.unsplash.com/photo-1486297678162-eb2a19b0a32d?w=800"),
        days_ago: 32,
        view_count: 180,
    },
    PostSeed {
        topic: "cheese",
        title: "Made my first homemade camembert",
        content: "After weeks of research and a lot of trial and error, I finally made my first batch of camembert from scratch! The bloomy rind developed beautifully — soft, white, and velvety. Inside is perfectly gooey at room temperature.\n\nThe process was surprisingly involved — pasteurizing the milk, adding cultures, waiting for the curds to set, salting, and then the long aging process. But the result is absolutely worth it.\n\nIf anyone's interested in trying cheesemaking, I'm happy to share notes and resources!",
        creator: "noblelion",
        tags: &["brie", "recipe"],
        image_url: Some("https://images.unsplash.com/photo-1559561853-2d5b0bf97b4a?w=800"),
        days_ago: 18,
        view_count: 155,
    },
    PostSeed {
        topic: "cheese",
        title: "Blue cheese: love it or hate it?",
        content: "I feel like there's no middle ground with blue cheese. People either worship the stuff or can't stand the smell. I'm firmly in the love camp — give me a chunk of Stilton with a drizzle of honey any day.\n\nWhat about you? Love it, hate it, or only in specific contexts?",
        creator: "oddotter",
        tags: &["blue", "tasting"],
        image_url: None,
        days_ago: 14,
        view_count: 94,
    },
    PostSeed {
        topic: "cheese",
        title: "Planning a cheese tasting party",
        content: "I want to host a small cheese tasting for a few friends and need help planning the lineup. I'm thinking:\n\n1. A mild fresh chèvre as a palate opener\n2. Aged gouda with those crunchy crystals\n3. A creamy brie de meaux\n4. Roquefort for the adventurous\n5. Aged manchego to finish\n\nPairing with honey, dried figs, and a crusty baguette. Any suggestions for additions or substitutions?",
        creator: "alice",
        tags: &["tasting", "recommendation"],
        image_url: None,
        days_ago: 7,
        view_count: 68,
    },
    // ── tech ──────────────────────────────────────────────────────────
    PostSeed {
        topic: "tech",
        title: "Building my first mechanical keyboard",
        content: "Just ordered all the parts for my first custom mechanical keyboard build!\n\n- PCB: TinyType S (40%)\n- Switches: Kailh Box Jades (clicky!)\n- Keycaps: SA profile in earthy tones\n- Case: walnut wood\n\nWish me luck with the soldering! I've watched about a dozen build guides and I'm feeling cautiously optimistic. The most nerve-wracking part is definitely the USB-C port — one wrong move and it's game over.",
        creator: "alice",
        tags: &["keyboards", "showcase"],
        image_url: Some("https://images.unsplash.com/photo-1587829741301-dc798b83add3?w=800"),
        days_ago: 30,
        view_count: 230,
    },
    PostSeed {
        topic: "tech",
        title: "Rust tip: using Result with axum",
        content: "Here's a quick pattern I've been using in my axum handlers:\n\nWrap your business logic in a service layer that returns Result<T, AppError>, then convert AppError into HTTP responses. Keeps your route handlers clean and testable.\n\n```rust\nasync fn get_post_handler(\n    State(pool): State<PgPool>,\n    Path(id): Path<i64>,\n) -> Result<Json<PostData>, AppError> {\n    let post = service::get_post(&pool, id).await?;\n    Ok(Json(post))\n}\n```\n\nThe key is implementing IntoResponse for your error enum so ? works seamlessly. Happy to expand on this if anyone's curious!",
        creator: "mouse",
        tags: &["rust", "tutorial"],
        image_url: None,
        days_ago: 28,
        view_count: 195,
    },
    PostSeed {
        topic: "tech",
        title: "My SFF PC build for 2025",
        content: "Just finished my small form factor PC build and I'm so happy with how it turned out. Fits in a 9.5L case with a full-size GPU inside.\n\nSpecs:\n- Ryzen 7 9800X3D\n- RTX 5070 (barely fits!)\n- 32GB DDR5-6000\n- Custom cables because the stock ones were a nightmare\n- All air-cooled — no AIO needed\n\nTemperatures are great — CPU tops out at 75°C under load and the GPU stays in the mid-60s. Total build took about 4 hours, with another 2 hours of cable management.",
        creator: "bravebadger",
        tags: &["hardware", "showcase"],
        image_url: Some("https://images.unsplash.com/photo-1591799264318-7e6ef8ddb7ea?w=800"),
        days_ago: 16,
        view_count: 112,
    },
    PostSeed {
        topic: "tech",
        title: "NixOS: first impressions after a month",
        content: "I took the plunge and switched to NixOS a month ago. Here are my unfiltered thoughts:\n\nPros: Declarative config is amazing once you wrap your head around it. Reproducible builds work exactly as promised. Rolling releases mean always-up-to-date packages.\n\nCons: The learning curve is brutal. Nix language is idiosyncratic. Documentation assumes you already understand the ecosystem. Debugging build failures can be cryptic.\n\nWould I recommend it? For servers, absolutely. For daily driving on a desktop, only if you enjoy tinkering with your OS config.",
        creator: "cleverfox",
        tags: &["webdev", "discussion"],
        image_url: None,
        days_ago: 11,
        view_count: 78,
    },
    PostSeed {
        topic: "tech",
        title: "What IDE do you use and why?",
        content: "Curious what everyone's using for their development environment. I've been a VS Code user for years but I'm thinking of switching things up.\n\nBeen eyeing:\n- Zed (heard great things about the performance)\n- Helix (modal editing without the Vim legacy cruft)\n- Just sticking with Neovim and terminal\n\nWhat's your setup and what made you choose it?",
        creator: "keenwolf",
        tags: &["discussion"],
        image_url: None,
        days_ago: 6,
        view_count: 90,
    },
    // ── art ───────────────────────────────────────────────────────────
    PostSeed {
        topic: "art",
        title: "My latest watercolor: sunset over the wheat field",
        content: "Just finished this little painting! It's a view from the edge of the burrow looking out at the wheat field during golden hour. The wheat stalks are taller than I expected to paint. Posted a pic — feedback welcome!",
        creator: "bob",
        tags: &["watercolor", "showcase"],
        image_url: Some("https://images.unsplash.com/photo-1513364776144-60967b0f800f?w=800"),
        days_ago: 27,
        view_count: 165,
    },
    PostSeed {
        topic: "art",
        title: "Drawn with cheese: edible art thread",
        content: "Has anyone else tried the nibble-and-draw technique? You basically sketch with different colored cheeses (cheddar orange, blue cheese veins, brie rind). The challenge is not eating your materials halfway through!",
        creator: "alice",
        tags: &["technique"],
        image_url: None,
        days_ago: 22,
        view_count: 140,
    },
    PostSeed {
        topic: "art",
        title: "Digital painting of the burrow at dawn",
        content: "Spent the weekend working on this digital landscape of our burrow at sunrise. I wanted to capture that soft golden light filtering through the grass blades and the dew on the leaves.\n\nDone in Procreate with custom brushes. About 8 hours of work spread across two days. The grass took forever to get right but I'm happy with how the depth turned out.",
        creator: "fancyrat",
        tags: &["digital", "showcase"],
        image_url: Some("https://images.unsplash.com/photo-1579783902614-a3fb3927b6a5?w=800"),
        days_ago: 9,
        view_count: 82,
    },
    PostSeed {
        topic: "art",
        title: "Polymer clay mouse sculpture",
        content: "My latest craft project — a tiny polymer clay mouse reading a book under a mushroom! The figure is about 3cm tall. Made with super sculpey and painted with acrylics.\n\nThe mushroom cap doubles as a little umbrella. Added some moss-textured clay for the base. Took about 6 hours total. What do you think?",
        creator: "proudbear",
        tags: &["traditional", "showcase"],
        image_url: None,
        days_ago: 4,
        view_count: 55,
    },
    // ── gaming ────────────────────────────────────────────────────────
    PostSeed {
        topic: "gaming",
        title: "Cheese Heist speedrun world record broken!",
        content: "The legendary runner QuickMouse just beat Cheese Heist Any% in 42:13! The new strat uses a wall clip in the kitchen level to skip the entire cat section. Absolutely insane run.",
        creator: "mouse",
        tags: &["speedrun"],
        image_url: None,
        days_ago: 25,
        view_count: 245,
    },
    PostSeed {
        topic: "gaming",
        title: "What are you playing this weekend?",
        content: "I'm diving back into Hollow Knight. Trying to finally beat the Pantheon of Hallownest. What's everyone else playing? Any hidden gem recommendations?",
        creator: "bob",
        tags: &["discussion"],
        image_url: None,
        days_ago: 20,
        view_count: 130,
    },
    PostSeed {
        topic: "gaming",
        title: "Balatro is consuming my life",
        content: "I picked up Balatro three days ago and I've already logged 20 hours. I can't stop. The poker roguelike gameplay is incredibly addictive — every run feels different and those high-scoring combos are pure dopamine.\n\nCurrent best hand: level 15 flush five with glass card multipliers. Scored over 1 million points on ante 11.\n\nSend help. Or better joker strategies.",
        creator: "swiftowl",
        tags: &["indie", "recommendation"],
        image_url: Some("https://images.unsplash.com/photo-1493711662062-fa541adb3fc8?w=800"),
        days_ago: 10,
        view_count: 105,
    },
    PostSeed {
        topic: "gaming",
        title: "Steam Deck vs ROG Ally: which one?",
        content: "I'm trying to decide between a Steam Deck and an ROG Ally for my next handheld. Steam Deck seems better for the price and has trackpads, but the Ally has better performance and Windows compatibility.\n\nHas anyone here used both? Which one do you prefer and why? I mainly play indie games and emulated classics, but I'd like the option to play newer AAA titles too.",
        creator: "keenwolf",
        tags: &["review", "discussion"],
        image_url: None,
        days_ago: 7,
        view_count: 88,
    },
    PostSeed {
        topic: "gaming",
        title: "My retro gaming corner setup",
        content: "Finally finished setting up my retro gaming corner! Got a CRT TV, an original NES, SNES, and a Sega Genesis all hooked up with a switcher. The scanlines on the CRT make 8-bit and 16-bit games look so much better than on modern screens.\n\nCurrent rotation: Super Metroid, Earthbound, and Sonic 3 & Knuckles. The nostalgia hit is real. Share your retro setups — I'd love to see them!",
        creator: "wildmoose",
        tags: &["retro", "showcase"],
        image_url: Some("https://images.unsplash.com/photo-1550745165-9bc0b252726f?w=800"),
        days_ago: 3,
        view_count: 73,
    },
    // ── books ─────────────────────────────────────────────────────────
    PostSeed {
        topic: "books",
        title: "Just finished 'The Mouse and the Motorcycle'",
        content: "Re-reading a childhood classic hits different as an adult. Beverly Cleary really understood the mouse perspective — the adventure, the danger, the thrill of riding a toy motorcycle. Any other mouse-lit recommendations?",
        creator: "alice",
        tags: &["fiction", "review"],
        image_url: None,
        days_ago: 24,
        view_count: 110,
    },
    PostSeed {
        topic: "books",
        title: "Cozy fantasy recommendations?",
        content: "I've been on a cozy fantasy kick lately and I'm running out of things to read. I've loved:\n- Legends & Lattes by Travis Baldree\n- The House in the Cerulean Sea by TJ Klune\n- A Psalm for the Wild-Built by Becky Chambers\n\nAnything similar I should pick up next? Low stakes, warm vibes, found family vibes preferred.",
        creator: "milddeer",
        tags: &["fantasy", "recommendation"],
        image_url: None,
        days_ago: 12,
        view_count: 79,
    },
    PostSeed {
        topic: "books",
        title: "Dune re-read — new insights as an adult",
        content: "I first read Dune as a teenager and loved it for the action and world-building. Re-reading it now in my 30s, I'm picking up so much more — the political commentary, ecological themes, and the nuanced take on messianic figures.\n\nPaul is much more tragic than I remembered. He knows the golden path leads to atrocity but walks it anyway because the alternative is extinction. Chilling stuff.\n\nAnyone else re-read classics and find completely different books?",
        creator: "cleverfox",
        tags: &["sci-fi", "discussion"],
        image_url: None,
        days_ago: 6,
        view_count: 62,
    },
    PostSeed {
        topic: "books",
        title: "Building a cozy reading corner",
        content: "I've been working on my reading nook: a little cardboard box lined with shredded paper (the soft kind!), a tiny LED lamp, and a thimble of chamomile tea. Perfect for rainy day reading. Show me your reading setups!",
        creator: "mouse",
        tags: &["discussion"],
        image_url: None,
        days_ago: 2,
        view_count: 45,
    },
    // ── off-topic ────────────────────────────────────────────────────
    PostSeed {
        topic: "off-topic",
        title: "How's the weather in your burrow?",
        content: "It's been raining non-stop here for three days. The good news is the garden is thriving. The bad news is everything is damp and I'm running low on dry nesting material.\n\nWhat's the weather like where you are? Anyone else dealing with spring floods or are you lucky enough to have sunny skies?",
        creator: "eagerhare",
        tags: &["weather", "discussion"],
        image_url: None,
        days_ago: 9,
        view_count: 56,
    },
    PostSeed {
        topic: "off-topic",
        title: "Pet tax thread: show us your pets!",
        content: "I know we're all mice here, but some of us have non-mouse friends too! I have a pet caterpillar named Munch who lives in a jar on my shelf. He's yellow and fuzzy and eats way more leaves than I expected.\n\nDrop pictures or descriptions of your pets! All creatures welcome.",
        creator: "gentleotter",
        tags: &["pets"],
        image_url: None,
        days_ago: 7,
        view_count: 95,
    },
    PostSeed {
        topic: "off-topic",
        title: "What music are you listening to?",
        content: "I've been on a huge lo-fi kick lately — perfect background music for reading, coding, or just chilling. Current favorites:\n- Jinsang (Life)\n- Idealism (Hiraeth)\n- Sleepy Fish (Chillin')\n\nWhat's everyone else listening to? Drop your current rotation!",
        creator: "oddotter",
        tags: &["music", "discussion"],
        image_url: None,
        days_ago: 4,
        view_count: 48,
    },
    // ── cooking ───────────────────────────────────────────────────────
    PostSeed {
        topic: "cooking",
        title: "Sourdough starter day 7 — it's alive!",
        content: "My sourdough starter finally doubled in size today! Day 7 and it smells like tangy, yeasty heaven. Named him Flour-dough. We're past the initial bacterial battle phase and he's bubbling beautifully.\n\nFeeding ratio is 1:2:2 (starter:flour:water) once a day with strong bread flour. Planning to bake my first loaf this weekend.\n\nAny sourdough veterans have tips for a first-time bake?",
        creator: "noblelion",
        tags: &["baking", "fermentation"],
        image_url: Some("https://images.unsplash.com/photo-1549931319-a545753467ef?w=800"),
        days_ago: 10,
        view_count: 88,
    },
    PostSeed {
        topic: "cooking",
        title: "One-pot pasta that changed my life",
        content: "I discovered the one-pot pasta method and I'll never go back. Everything cooks in a single pan — pasta, sauce, vegetables, all together. Less dishes, more flavor because the starch thickens the sauce naturally.\n\nMy go-to recipe:\n- Cherry tomatoes, halved\n- Garlic, sliced\n- Basil leaves\n- Olive oil\n- Dried pasta\n- Vegetable broth\n\nCook everything together for about 12 minutes, finish with parmesan and fresh basil. Done in under 20 minutes with one dish to wash.",
        creator: "milddeer",
        tags: &["recipe"],
        image_url: None,
        days_ago: 5,
        view_count: 67,
    },
    PostSeed {
        topic: "cooking",
        title: "My fermentation station setup",
        content: "I've gone down the fermentation rabbit hole and my kitchen counter is starting to look like a science lab. Current projects:\n\n- Kimchi (day 4, tasting amazing)\n- Kombucha (continuous brew, second fermentation with mango)\n- Sauerkraut (week 2, perfectly crunchy)\n- Hot sauce (fermented habanero + garlic, going for 3 weeks)\n\nThe smell is... potent. But the results are incredible. Anyone else into fermenting? Would love to swap recipes!",
        creator: "bob",
        tags: &["fermentation", "showcase"],
        image_url: Some("https://images.unsplash.com/photo-1466637574441-749b8f19452f?w=800"),
        days_ago: 2,
        view_count: 42,
    },
];

// ── Comments ──────────────────────────────────────────────────────────

pub static COMMENTS: &[CommentSeed] = &[
    // ── Comments indices 0-14 ────────────────────────────────────────
    CommentSeed { post_title: "Welcome to TwoMice!", content: "Hey mouse, thanks for setting this up! Really looking forward to exploring everything.", sender: "bob", days_ago: 44 },
    CommentSeed { post_title: "Welcome to TwoMice!", content: "This interface is so clean. Love the cozy vibe already!", sender: "alice", days_ago: 44 },
    CommentSeed { post_title: "Community Guidelines", content: "Short and sweet. These rules cover everything important.", sender: "mouse", days_ago: 43 },
    CommentSeed { post_title: "Best cheddar I've ever nibbled", content: "Was it the clothbound kind from the artisan stall? Those are always incredible.", sender: "alice", days_ago: 34 },
    CommentSeed { post_title: "Best cheddar I've ever nibbled", content: "You have to tell us where you got this! I've been on the hunt for good cheddar.", sender: "mouse", days_ago: 34 },
    CommentSeed { post_title: "Best cheddar I've ever nibbled", content: "The crystals are the best part. That crunch is absolutely unmatched.", sender: "bob", days_ago: 33 },
    CommentSeed { post_title: "Gouda vs Edam: the ultimate showdown", content: "Aged Gouda is unbeatable. Those crunchy crystals win every time for me.", sender: "bob", days_ago: 31 },
    CommentSeed { post_title: "Gouda vs Edam: the ultimate showdown", content: "Smoked Gouda with a drizzle of honey is my go-to party snack. Highly recommend.", sender: "alice", days_ago: 31 },
    CommentSeed { post_title: "Building my first mechanical keyboard", content: "Box Jades are so loud! I love the choice, great way to start the hobby.", sender: "mouse", days_ago: 29 },
    CommentSeed { post_title: "Building my first mechanical keyboard", content: "Please share a sound test once it's done! Clicky builds are the most satisfying.", sender: "bob", days_ago: 29 },
    CommentSeed { post_title: "Rust tip: using Result with axum", content: "Great pattern. I use a similar approach with thiserror for clean error handling code.", sender: "alice", days_ago: 27 },
    CommentSeed { post_title: "My latest watercolor: sunset over the wheat field", content: "The warm golden tones are beautiful! How did you blend the sky so smoothly?", sender: "mouse", days_ago: 26 },
    CommentSeed { post_title: "Drawn with cheese: edible art thread", content: "I tried this and ate my entire art supply within minutes. Worth every bite though.", sender: "bob", days_ago: 21 },
    CommentSeed { post_title: "Cheese Heist speedrun world record broken!", content: "That wall clip in the kitchen level is absolutely insane. Speedrunners are incredible.", sender: "alice", days_ago: 24 },
    CommentSeed { post_title: "Just finished 'The Mouse and the Motorcycle'", content: "If you loved that, definitely check out Redwall next. Epic mouse fantasy at its finest.", sender: "bob", days_ago: 23 },

    // ── New comments (indices 15-99) ──────────────────────────────────
    // announcements
    CommentSeed { post_title: "Welcome to TwoMice!", content: "Glad to be part of this from the start!", sender: "swiftowl", days_ago: 44 },
    CommentSeed { post_title: "Welcome to TwoMice!", content: "This place looks amazing. Great work mouse!", sender: "bravebadger", days_ago: 43 },
    CommentSeed { post_title: "Community Guidelines", content: "Seems reasonable. Thanks for keeping things civil.", sender: "calmpanda", days_ago: 43 },
    CommentSeed { post_title: "Community Guidelines", content: "Number 4 is the most important rule!", sender: "eagerhare", days_ago: 43 },
    CommentSeed { post_title: "Site Updates — June 2025", content: "The cooking board is exactly what I needed!", sender: "milddeer", days_ago: 9 },
    CommentSeed { post_title: "Site Updates — June 2025", content: "Tag filtering sounds useful. How does it work?", sender: "keenwolf", days_ago: 9 },

    // introductions
    CommentSeed { post_title: "Hi everyone, I'm new here!", content: "Welcome! Best way to get involved is to just jump in and post. What kind of games do you play?", sender: "swiftowl", days_ago: 11 },
    CommentSeed { post_title: "Hi everyone, I'm new here!", content: "Welcome happybunny! We're glad you joined!", sender: "alice", days_ago: 11 },
    CommentSeed { post_title: "Hi everyone, I'm new here!", content: "If you like fantasy novels, check out the books board!", sender: "milddeer", days_ago: 11 },
    CommentSeed { post_title: "Long-time lurker, finally posting", content: "A fellow keyboard enthusiast! What's your current setup?", sender: "alice", days_ago: 7 },
    CommentSeed { post_title: "Long-time lurker, finally posting", content: "Welcome! Sourdough and keyboards, you have great taste.", sender: "noblelion", days_ago: 7 },
    CommentSeed { post_title: "Greetings from the UK burrow!", content: "Welcome fellow UK mouse! What part are you from?", sender: "eagerhare", days_ago: 5 },
    CommentSeed { post_title: "Greetings from the UK burrow!", content: "Cheddar IS life. You'll fit in perfectly here.", sender: "bob", days_ago: 5 },

    // general
    CommentSeed { post_title: "What's everyone snacking on today?", content: "Just had some manchego with quince paste. Highly recommend!", sender: "noblelion", days_ago: 14 },
    CommentSeed { post_title: "What's everyone snacking on today?", content: "I'm on a brie and cranberry sauce kick lately. So good.", sender: "eagerhare", days_ago: 14 },
    CommentSeed { post_title: "What's everyone snacking on today?", content: "Dried apple slices and sharp cheddar. Classic combo.", sender: "wildmoose", days_ago: 14 },
    CommentSeed { post_title: "Forum rules and guidelines", content: "Can we add a rule about stealing cheese from other mice's plates?", sender: "sillygoat", days_ago: 41 },
    CommentSeed { post_title: "Hot take: mornings are better than nights", content: "Hard disagree. Nights are when the adventures happen!", sender: "wildmoose", days_ago: 13 },
    CommentSeed { post_title: "Hot take: mornings are better than nights", content: "I'm a morning mouse too! The world is so peaceful before everyone wakes up.", sender: "calmpanda", days_ago: 13 },
    CommentSeed { post_title: "Hot take: mornings are better than nights", content: "Why not both? Early morning AND late night are the best parts of the day.", sender: "cleverfox", days_ago: 13 },
    CommentSeed { post_title: "What's your favorite mouse pun?", content: "I'm not sure if this counts, but 'squeak' is a very versatile word.", sender: "calmpanda", days_ago: 6 },
    CommentSeed { post_title: "What's your favorite mouse pun?", content: "I'm cheesy, therefore I am.", sender: "sillygoat", days_ago: 6 },
    CommentSeed { post_title: "What's your favorite mouse pun?", content: "These puns are making me feel rather... mousederstood.", sender: "cleverfox", days_ago: 6 },
    CommentSeed { post_title: "Should we have a community event?", content: "A gaming night sounds great! We could all try a multiplayer game together.", sender: "keenwolf", days_ago: 4 },
    CommentSeed { post_title: "Should we have a community event?", content: "Cheese tasting weekend gets my vote!", sender: "bob", days_ago: 4 },
    CommentSeed { post_title: "Should we have a community event?", content: "Why not both? One weekend for gaming, another for cheese?", sender: "swiftowl", days_ago: 4 },
    CommentSeed { post_title: "Should we have a community event?", content: "I can help organize if needed. Count me in!", sender: "bravebadger", days_ago: 4 },

    // cheese
    CommentSeed { post_title: "Best cheddar I've ever nibbled", content: "Those crystals are calcium lactate. A sign of well-aged cheese!", sender: "noblelion", days_ago: 34 },
    CommentSeed { post_title: "Best cheddar I've ever nibbled", content: "Please say this is from Neal's Yard Dairy. Their cheddar is legendary.", sender: "eagerhare", days_ago: 34 },
    CommentSeed { post_title: "Gouda vs Edam: the ultimate showdown", content: "Team Gouda forever. Three-year aged is a religious experience.", sender: "noblelion", days_ago: 31 },
    CommentSeed { post_title: "Gouda vs Edam: the ultimate showdown", content: "Edam is underrated. It's the perfect snacking cheese — firm, mild, not too greasy.", sender: "calmpanda", days_ago: 31 },
    CommentSeed { post_title: "Made my first homemade camembert", content: "That's incredible! Cheesemaking is on my bucket list. How did you handle the humidity during aging?", sender: "calmpanda", days_ago: 17 },
    CommentSeed { post_title: "Made my first homemade camembert", content: "I've been making cheese for years and this is impressive for a first batch. Well done!", sender: "deepmole", days_ago: 17 },
    CommentSeed { post_title: "Made my first homemade camembert", content: "Would love to see pictures of the cross-section!", sender: "eagerhare", days_ago: 17 },
    CommentSeed { post_title: "Blue cheese: love it or hate it?", content: "Love it on a burger, hate it on its own. Is that allowed?", sender: "keenwolf", days_ago: 13 },
    CommentSeed { post_title: "Blue cheese: love it or hate it?", content: "Stilton with port is one of life's greatest pleasures.", sender: "noblelion", days_ago: 13 },
    CommentSeed { post_title: "Blue cheese: love it or hate it?", content: "I want to like it but it tastes like... feet. Am I doing it wrong?", sender: "gentleotter", days_ago: 13 },
    CommentSeed { post_title: "Planning a cheese tasting party", content: "Great lineup! I'd suggest adding a washed-rind cheese like Taleggio for variety.", sender: "bob", days_ago: 6 },
    CommentSeed { post_title: "Planning a cheese tasting party", content: "Make sure you serve them from mildest to strongest!", sender: "noblelion", days_ago: 6 },
    CommentSeed { post_title: "Planning a cheese tasting party", content: "I'm available if you need a taste tester!", sender: "jollyfrog", days_ago: 6 },

    // tech
    CommentSeed { post_title: "Building my first mechanical keyboard", content: "Box Jades are LOUD. Hope you don't have roommates!", sender: "bravebadger", days_ago: 29 },
    CommentSeed { post_title: "Building my first mechanical keyboard", content: "Would love to see the final build! 40% boards are so fun.", sender: "keenwolf", days_ago: 29 },
    CommentSeed { post_title: "Rust tip: using Result with axum", content: "We use this pattern in production and it's rock solid. Good writeup!", sender: "keenwolf", days_ago: 27 },
    CommentSeed { post_title: "Rust tip: using Result with axum", content: "One tip: use #[derive(IntoResponse)] from axum for your error types. Saves boilerplate.", sender: "bravebadger", days_ago: 27 },
    CommentSeed { post_title: "My SFF PC build for 2025", content: "Those custom cables make all the difference in SFF builds. Where did you get yours?", sender: "alice", days_ago: 15 },
    CommentSeed { post_title: "My SFF PC build for 2025", content: "75C on air with a 9800X3D is impressive. What cooler are you using?", sender: "keenwolf", days_ago: 15 },
    CommentSeed { post_title: "NixOS: first impressions after a month", content: "Your pros and cons perfectly capture the NixOS experience. I switched two years ago and never looked back.", sender: "tinybat", days_ago: 10 },
    CommentSeed { post_title: "NixOS: first impressions after a month", content: "The documentation issue is real. I found the NixOS wiki and nix.dev to be the most helpful.", sender: "bravebadger", days_ago: 10 },
    CommentSeed { post_title: "What IDE do you use and why?", content: "VS Code with vim keybindings is my sweet spot. Best of both worlds.", sender: "cleverfox", days_ago: 5 },
    CommentSeed { post_title: "What IDE do you use and why?", content: "Zed is genuinely fast. I switched from VS Code and haven't looked back.", sender: "swiftowl", days_ago: 5 },
    CommentSeed { post_title: "What IDE do you use and why?", content: "Neovim for life. Once you go modal you can't go back.", sender: "keenwolf", days_ago: 5 },

    // art
    CommentSeed { post_title: "My latest watercolor: sunset over the wheat field", content: "The warm tones are gorgeous! How did you get the sky gradient so smooth?", sender: "fancyrat", days_ago: 26 },
    CommentSeed { post_title: "Drawn with cheese: edible art thread", content: "I tried this after seeing your post. Used a sharp cheddar for the sunset. Lasted 5 minutes before I ate it.", sender: "sillygoat", days_ago: 21 },
    CommentSeed { post_title: "Digital painting of the burrow at dawn", content: "Procreate custom brushes are a game changer. Which brush pack did you use for the grass?", sender: "bob", days_ago: 8 },
    CommentSeed { post_title: "Digital painting of the burrow at dawn", content: "The dew drops look so realistic! Great work.", sender: "royalcat", days_ago: 8 },
    CommentSeed { post_title: "Polymer clay mouse sculpture", content: "This is adorable! Do you sell these or take commissions?", sender: "gentleotter", days_ago: 3 },
    CommentSeed { post_title: "Polymer clay mouse sculpture", content: "The mushroom umbrella is such a cute detail! How did you get the spots so even?", sender: "fancyrat", days_ago: 3 },

    // gaming
    CommentSeed { post_title: "Cheese Heist speedrun world record broken!", content: "That kitchen clip has been known about for a month! Surprised it took this long for someone to use it in a record run.", sender: "keenwolf", days_ago: 24 },
    CommentSeed { post_title: "Cheese Heist speedrun world record broken!", content: "QuickMouse is an absolute legend. His movement is so clean.", sender: "swiftowl", days_ago: 24 },
    CommentSeed { post_title: "What are you playing this weekend?", content: "I've been playing Hades 2 non-stop. The early access content is already incredible.", sender: "swiftowl", days_ago: 19 },
    CommentSeed { post_title: "What are you playing this weekend?", content: "Working through the Mass Effect trilogy for the first time. How did I miss this?", sender: "gentleotter", days_ago: 19 },
    CommentSeed { post_title: "Balatro is consuming my life", content: "Try the Anaglyph deck — the polychrome multiplier stacking is insane!", sender: "wildmoose", days_ago: 9 },
    CommentSeed { post_title: "Balatro is consuming my life", content: "I'm stuck on gold stake difficulty. Any tips?", sender: "eagerhare", days_ago: 9 },
    CommentSeed { post_title: "Balatro is consuming my life", content: "Focus on building toward a specific hand type early and grab jokers that support it.", sender: "swiftowl", days_ago: 9 },
    CommentSeed { post_title: "Steam Deck vs ROG Ally: which one?", content: "I have both. Steam Deck for battery life and UI, Ally for raw performance. Pick your priority.", sender: "bravebadger", days_ago: 6 },
    CommentSeed { post_title: "Steam Deck vs ROG Ally: which one?", content: "Steam Deck is better value. The trackpads are surprisingly useful for strategy games.", sender: "wildmoose", days_ago: 6 },
    CommentSeed { post_title: "My retro gaming corner setup", content: "That CRT is gorgeous! What model is it?", sender: "keenwolf", days_ago: 2 },
    CommentSeed { post_title: "My retro gaming corner setup", content: "Earthbound on a CRT is the definitive way to play it. Respect.", sender: "sillygoat", days_ago: 2 },

    // books
    CommentSeed { post_title: "Just finished 'The Mouse and the Motorcycle'", content: "Redwall is the obvious next step. Also check out The Tale of Despereaux!", sender: "calmpanda", days_ago: 23 },
    CommentSeed { post_title: "Cozy fantasy recommendations?", content: "Try 'A Wizard's Guide to Defensive Baking' by T. Kingfisher. Cozy, funny, and has a great protagonist.", sender: "cleverfox", days_ago: 11 },
    CommentSeed { post_title: "Cozy fantasy recommendations?", content: "Adding all of these to my reading list. Thanks for the recs!", sender: "happybunny", days_ago: 11 },
    CommentSeed { post_title: "Dune re-read — new insights as an adult", content: "I had the exact same experience re-reading Dune. The political depth went completely over my head as a teen.", sender: "proudbear", days_ago: 5 },
    CommentSeed { post_title: "Dune re-read — new insights as an adult", content: "Herbert's warning about charismatic leaders is more relevant now than ever.", sender: "tinybat", days_ago: 5 },
    CommentSeed { post_title: "Building a cozy reading corner", content: "The thimble of tea is such a lovely touch! I use an acorn cap for mine.", sender: "calmpanda", days_ago: 1 },
    CommentSeed { post_title: "Building a cozy reading corner", content: "I need to step up my reading nook game. Mine is just a shoebox with a napkin.", sender: "happybunny", days_ago: 1 },

    // off-topic
    CommentSeed { post_title: "How's the weather in your burrow?", content: "Sunny and warm here! Finally got out of the rainy season.", sender: "jollyfrog", days_ago: 8 },
    CommentSeed { post_title: "How's the weather in your burrow?", content: "Same here — three days of rain. My compost pile is very happy though.", sender: "milddeer", days_ago: 8 },
    CommentSeed { post_title: "Pet tax thread: show us your pets!", content: "I have a pet beetle named Sir Nibblesworth. He eats rotting wood and minding his own business.", sender: "deepmole", days_ago: 6 },
    CommentSeed { post_title: "Pet tax thread: show us your pets!", content: "Munch the caterpillar sounds adorable. What does he look like when he's fuzzy?", sender: "eagerhare", days_ago: 6 },
    CommentSeed { post_title: "What music are you listening to?", content: "I've been deep into ambient electronic lately. Brian Eno's 'Music for Airports' on repeat.", sender: "calmpanda", days_ago: 3 },
    CommentSeed { post_title: "What music are you listening to?", content: "Classical piano while reading, synthwave while coding. No in-between.", sender: "cleverfox", days_ago: 3 },

    // cooking
    CommentSeed { post_title: "Sourdough starter day 7 — it's alive!", content: "Flour-dough is an excellent name. My starter is named 'Yeastie Boys'.", sender: "bob", days_ago: 9 },
    CommentSeed { post_title: "Sourdough starter day 7 — it's alive!", content: "Tip: don't bake your first loaf when you're hungry. The anticipation is brutal.", sender: "milddeer", days_ago: 9 },
    CommentSeed { post_title: "One-pot pasta that changed my life", content: "Try adding a splash of pasta water at the end. It emulsifies the sauce beautifully.", sender: "noblelion", days_ago: 4 },
    CommentSeed { post_title: "One-pot pasta that changed my life", content: "One-pot meals are my weekday savior. This sounds delicious!", sender: "eagerhare", days_ago: 4 },
    CommentSeed { post_title: "My fermentation station setup", content: "The hot sauce sounds amazing! What ratio of habanero to garlic do you use?", sender: "noblelion", days_ago: 1 },
    CommentSeed { post_title: "My fermentation station setup", content: "Kombucha is my current obsession too. The mango second ferment is genius!", sender: "milddeer", days_ago: 1 },
];

// ── Replies ──────────────────────────────────────────────────────────

pub static REPLIES: &[ReplySeed] = &[
    // ── Replies indices 0-5 ──────────────────────────────────────────
    ReplySeed { post_title: "Welcome to TwoMice!", comment_idx: 0, parent_reply_idx: None, content: "Great to see you here bob! Feel free to explore all the boards.", sender: "mouse", days_ago: 44 },
    ReplySeed { post_title: "Welcome to TwoMice!", comment_idx: 0, parent_reply_idx: None, content: "This is going to be a wonderful community. I can already feel it!", sender: "alice", days_ago: 44 },
    ReplySeed { post_title: "Community Guidelines", comment_idx: 2, parent_reply_idx: None, content: "No confusion here. Clear and straightforward, exactly how rules should be.", sender: "bob", days_ago: 42 },
    ReplySeed { post_title: "Gouda vs Edam: the ultimate showdown", comment_idx: 6, parent_reply_idx: None, content: "The 18-month aged Gouda from the deli counter is absolutely worth trying.", sender: "mouse", days_ago: 31 },
    ReplySeed { post_title: "Building my first mechanical keyboard", comment_idx: 8, parent_reply_idx: None, content: "Can't wait for that sound test! Clicky builds are the most satisfying.", sender: "alice", days_ago: 29 },
    ReplySeed { post_title: "Cheese Heist speedrun world record broken!", comment_idx: 13, parent_reply_idx: None, content: "They always patch the best strats eventually. Enjoy the record while it lasts!", sender: "mouse", days_ago: 24 },

    // ── New replies (indices 6-59) ────────────────────────────────────
    // Thread under "Welcome to TwoMice!" — reply chain
    ReplySeed { post_title: "Welcome to TwoMice!", comment_idx: 0, parent_reply_idx: Some(0), content: "Thanks mouse! Excited to be part of this community.", sender: "bob", days_ago: 43 },
    ReplySeed { post_title: "Welcome to TwoMice!", comment_idx: 0, parent_reply_idx: Some(6), content: "You've already been a great member bob! Thanks for being here.", sender: "mouse", days_ago: 43 },
    ReplySeed { post_title: "Welcome to TwoMice!", comment_idx: 0, parent_reply_idx: Some(7), content: "This is the friendliest place on the internet. I'm so glad I joined.", sender: "alice", days_ago: 42 },

    // Thread under "Best cheddar" — cheese discussion
    ReplySeed { post_title: "Best cheddar I've ever nibbled", comment_idx: 4, parent_reply_idx: None, content: "It was from a small artisan vendor at the weekend market. I'll grab the name next time I'm there!", sender: "bob", days_ago: 33 },
    ReplySeed { post_title: "Best cheddar I've ever nibbled", comment_idx: 4, parent_reply_idx: Some(9), content: "Please do! I've been hunting for good cheddar since I moved burrows.", sender: "mouse", days_ago: 33 },
    ReplySeed { post_title: "Best cheddar I've ever nibbled", comment_idx: 4, parent_reply_idx: Some(10), content: "There's a place called The Cheese Wheel on Burrow Street that has amazing aged cheddar.", sender: "alice", days_ago: 32 },
    ReplySeed { post_title: "Best cheddar I've ever nibbled", comment_idx: 4, parent_reply_idx: Some(11), content: "Seconding The Cheese Wheel! Their Montgomery's Cheddar is the best I've had outside the UK.", sender: "noblelion", days_ago: 32 },
    ReplySeed { post_title: "Best cheddar I've ever nibbled", comment_idx: 5, parent_reply_idx: None, content: "I found a clothbound cheddar at the deli counter and it blew my mind. Those crystals!", sender: "calmpanda", days_ago: 32 },

    // Thread under "Gouda vs Edam"
    ReplySeed { post_title: "Gouda vs Edam: the ultimate showdown", comment_idx: 6, parent_reply_idx: Some(3), content: "18-month aged is good, but have you had 3-year aged? It's a completely different experience.", sender: "noblelion", days_ago: 30 },
    ReplySeed { post_title: "Gouda vs Edam: the ultimate showdown", comment_idx: 7, parent_reply_idx: None, content: "Smoked Gouda with honey is my go-to party snack. Always a hit!", sender: "eagerhare", days_ago: 30 },
    ReplySeed { post_title: "Gouda vs Edam: the ultimate showdown", comment_idx: 7, parent_reply_idx: Some(14), content: "Try it with a drizzle of balsamic glaze too. Takes it to another level.", sender: "noblelion", days_ago: 30 },

    // Thread under "Homemade camembert"
    ReplySeed { post_title: "Made my first homemade camembert", comment_idx: 44, parent_reply_idx: None, content: "I use a plastic storage box with a lid slightly cracked open. Humidity stays around 85% with a damp cloth inside.", sender: "noblelion", days_ago: 17 },
    ReplySeed { post_title: "Made my first homemade camembert", comment_idx: 44, parent_reply_idx: Some(16), content: "Thanks for the tip! I'll try the damp cloth method.", sender: "calmpanda", days_ago: 16 },
    ReplySeed { post_title: "Made my first homemade camembert", comment_idx: 45, parent_reply_idx: None, content: "Coming from someone with years of experience, that means a lot!", sender: "noblelion", days_ago: 16 },

    // Thread under "Mechanical keyboard"
    ReplySeed { post_title: "Building my first mechanical keyboard", comment_idx: 8, parent_reply_idx: Some(4), content: "I live alone so the louder the better! Going for maximum clickiness.", sender: "alice", days_ago: 28 },
    ReplySeed { post_title: "Building my first mechanical keyboard", comment_idx: 8, parent_reply_idx: Some(19), content: "Respect. Box Jades are the loudest clicky switch I've ever used. Your neighbors will know.", sender: "bravebadger", days_ago: 28 },
    ReplySeed { post_title: "Building my first mechanical keyboard", comment_idx: 9, parent_reply_idx: None, content: "I'll post a sound test when it's done! Thinking of lubing the stabilizers too.", sender: "alice", days_ago: 28 },

    // Thread under "Rust tip"
    ReplySeed { post_title: "Rust tip: using Result with axum", comment_idx: 10, parent_reply_idx: None, content: "Do you use thiserror or anyhow for your error types in application code?", sender: "bravebadger", days_ago: 26 },
    ReplySeed { post_title: "Rust tip: using Result with axum", comment_idx: 10, parent_reply_idx: Some(22), content: "thiserror for library errors, anyhow for application code. They serve different purposes!", sender: "mouse", days_ago: 26 },
    ReplySeed { post_title: "Rust tip: using Result with axum", comment_idx: 55, parent_reply_idx: None, content: "Definitely agree on the IntoResponse derive. We use it with all our error types now.", sender: "mouse", days_ago: 26 },

    // Thread under "Watercolor"
    ReplySeed { post_title: "My latest watercolor: sunset over the wheat field", comment_idx: 11, parent_reply_idx: None, content: "Wet-on-wet technique for the sky, then layered the warm colors while it was still damp. Lots of practice!", sender: "bob", days_ago: 25 },
    ReplySeed { post_title: "My latest watercolor: sunset over the wheat field", comment_idx: 11, parent_reply_idx: Some(25), content: "Wet-on-wet is so tricky to master! The colors in your painting blended perfectly.", sender: "fancyrat", days_ago: 25 },
    ReplySeed { post_title: "My latest watercolor: sunset over the wheat field", comment_idx: 11, parent_reply_idx: Some(26), content: "Thank you! It took about 5 attempts to get the sky right. Water is unforgiving!", sender: "bob", days_ago: 24 },

    // Thread under "Cheese Heist speedrun"
    ReplySeed { post_title: "Cheese Heist speedrun world record broken!", comment_idx: 13, parent_reply_idx: Some(5), content: "The patch is coming in v2.4 according to the dev blog. Enjoy the WR while it stands!", sender: "keenwolf", days_ago: 23 },
    ReplySeed { post_title: "Cheese Heist speedrun world record broken!", comment_idx: 13, parent_reply_idx: Some(28), content: "QuickMouse is already practicing a backup route. He'll retain the record mark my words.", sender: "swiftowl", days_ago: 23 },

    // Thread under "Balatro"
    ReplySeed { post_title: "Balatro is consuming my life", comment_idx: 73, parent_reply_idx: None, content: "Anaglyph deck with polychrome stacking is the most fun I've had in Balatro. Great recommendation!", sender: "swiftowl", days_ago: 8 },
    ReplySeed { post_title: "Balatro is consuming my life", comment_idx: 73, parent_reply_idx: Some(30), content: "Right?! The first time you see a polychrome multiplied by another polychrome... chef's kiss.", sender: "wildmoose", days_ago: 8 },
    ReplySeed { post_title: "Balatro is consuming my life", comment_idx: 74, parent_reply_idx: None, content: "Focus on flush houses or five-of-a-kind. Jokers that multiply per card scored are key.", sender: "swiftowl", days_ago: 8 },

    // Thread under "SFF PC build"
    ReplySeed { post_title: "My SFF PC build for 2025", comment_idx: 56, parent_reply_idx: None, content: "Got custom cables from Dreambigbyray on Etsy. Highly recommend!", sender: "bravebadger", days_ago: 14 },
    ReplySeed { post_title: "My SFF PC build for 2025", comment_idx: 57, parent_reply_idx: None, content: "Using a Thermalright AXP-90 X47 Full Copper. Surprisingly effective for a low profile cooler!", sender: "bravebadger", days_ago: 14 },

    // Thread under "Cozy fantasy"
    ReplySeed { post_title: "Cozy fantasy recommendations?", comment_idx: 80, parent_reply_idx: None, content: "T. Kingfisher is incredible. Also try 'The Very Secret Society of Irregular Witches'!", sender: "milddeer", days_ago: 10 },
    ReplySeed { post_title: "Cozy fantasy recommendations?", comment_idx: 80, parent_reply_idx: Some(35), content: "Seconding Irregular Witches! Such a warm, comforting read.", sender: "calmpanda", days_ago: 10 },

    // Thread under "Dune re-read"
    ReplySeed { post_title: "Dune re-read — new insights as an adult", comment_idx: 81, parent_reply_idx: None, content: "The way Herbert deconstructs the hero archetype is genius. Paul is a warning, not a role model.", sender: "cleverfox", days_ago: 4 },
    ReplySeed { post_title: "Dune re-read — new insights as an adult", comment_idx: 82, parent_reply_idx: None, content: "It's eerie how relevant the political themes are today. The spice as oil analogy is perfect.", sender: "milddeer", days_ago: 4 },

    // Thread under "Sourdough starter"
    ReplySeed { post_title: "Sourdough starter day 7 — it's alive!", comment_idx: 89, parent_reply_idx: None, content: "Yeastie Boys! That's amazing. Great name.", sender: "noblelion", days_ago: 8 },
    ReplySeed { post_title: "Sourdough starter day 7 — it's alive!", comment_idx: 89, parent_reply_idx: None, content: "I name all my ferments too. My kombucha SCOBY is called 'Bryce'.", sender: "oddotter", days_ago: 8 },
    ReplySeed { post_title: "Sourdough starter day 7 — it's alive!", comment_idx: 90, parent_reply_idx: None, content: "The anticipation is real. I've been staring at my starter every hour waiting for it to peak.", sender: "noblelion", days_ago: 8 },

    // Thread under "Retro gaming corner"
    ReplySeed { post_title: "My retro gaming corner setup", comment_idx: 77, parent_reply_idx: None, content: "It's a Sony Trinitron KV-14CT1U. 14 inches of pure nostalgia!", sender: "wildmoose", days_ago: 1 },
    ReplySeed { post_title: "My retro gaming corner setup", comment_idx: 77, parent_reply_idx: Some(41), content: "Trinitrons are legendary. The aperture grille makes such a difference for 240p content.", sender: "keenwolf", days_ago: 1 },

    // Thread under "Pet tax"
    ReplySeed { post_title: "Pet tax thread: show us your pets!", comment_idx: 86, parent_reply_idx: None, content: "Sir Nibblesworth is a noble name for a beetle. Does he have a tiny crown?", sender: "gentleotter", days_ago: 5 },
    ReplySeed { post_title: "Pet tax thread: show us your pets!", comment_idx: 87, parent_reply_idx: None, content: "He's bright green and fuzzy like a kiwi fruit! I'll try to get a photo.", sender: "eagerhare", days_ago: 5 },

    // Thread under "What's your favorite pun?"
    ReplySeed { post_title: "What's your favorite mouse pun?", comment_idx: 36, parent_reply_idx: None, content: "I was going to make a cheese pun but it's too gouda be true.", sender: "bob", days_ago: 5 },
    ReplySeed { post_title: "What's your favorite mouse pun?", comment_idx: 36, parent_reply_idx: Some(46), content: "That was so bad it's actually good. Take my upvote.", sender: "sillygoat", days_ago: 5 },
    ReplySeed { post_title: "What's your favorite mouse pun?", comment_idx: 38, parent_reply_idx: None, content: "I see what you did there. Mousederstood indeed!", sender: "jollyfrog", days_ago: 5 },

    // Thread under "Blue cheese"
    ReplySeed { post_title: "Blue cheese: love it or hate it?", comment_idx: 50, parent_reply_idx: None, content: "Totally allowed! Blue cheese crumbled on a burger is perfection.", sender: "oddotter", days_ago: 12 },
    ReplySeed { post_title: "Blue cheese: love it or hate it?", comment_idx: 52, parent_reply_idx: None, content: "Start with a mild gorgonzola dolce. It's creamy and much less intense. Build up from there!", sender: "noblelion", days_ago: 12 },

    // Thread under "Community event"
    ReplySeed { post_title: "Should we have a community event?", comment_idx: 40, parent_reply_idx: None, content: "I'd be down for a gaming night! We could play something free like Brawlhalla.", sender: "sillygoat", days_ago: 3 },
    ReplySeed { post_title: "Should we have a community event?", comment_idx: 41, parent_reply_idx: None, content: "Cheese tasting weekend would be amazing! We could all post reviews of the same cheese.", sender: "alice", days_ago: 3 },

    // Thread under "Fermentation station"
    ReplySeed { post_title: "My fermentation station setup", comment_idx: 97, parent_reply_idx: None, content: "I use about 5 habaneros to 3 garlic cloves per cup of brine. Adjust based on your heat tolerance!", sender: "bob", days_ago: 1 },
    ReplySeed { post_title: "My fermentation station setup", comment_idx: 97, parent_reply_idx: Some(52), content: "That sounds fiery! I'll start with a milder ratio and work my way up.", sender: "noblelion", days_ago: 1 },
    ReplySeed { post_title: "My fermentation station setup", comment_idx: 98, parent_reply_idx: None, content: "The mango kombucha is surprisingly easy! Just add mango puree to the second ferment and wait 3 days.", sender: "bob", days_ago: 1 },

    // Thread under "NixOS"
    ReplySeed { post_title: "NixOS: first impressions after a month", comment_idx: 58, parent_reply_idx: None, content: "Two years in and still learning. That's the beauty of NixOS — there's always more to discover.", sender: "tinybat", days_ago: 9 },
    ReplySeed { post_title: "NixOS: first impressions after a month", comment_idx: 59, parent_reply_idx: None, content: "The nix.dev tutorials helped me a lot. Also the Nix Pills series is essential reading.", sender: "cleverfox", days_ago: 9 },
];

// ── Post votes ────────────────────────────────────────────────────────

pub static POST_VOTES: &[VoteSeed] = &[
    VoteSeed { post_title: "Welcome to TwoMice!", voter: "alice", direction: 1 },
    VoteSeed { post_title: "Welcome to TwoMice!", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Welcome to TwoMice!", voter: "swiftowl", direction: 1 },
    VoteSeed { post_title: "Welcome to TwoMice!", voter: "bravebadger", direction: 1 },
    VoteSeed { post_title: "Welcome to TwoMice!", voter: "eagerhare", direction: 1 },
    VoteSeed { post_title: "Welcome to TwoMice!", voter: "calmpanda", direction: 1 },
    VoteSeed { post_title: "Welcome to TwoMice!", voter: "cleverfox", direction: 1 },

    VoteSeed { post_title: "Community Guidelines", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "Community Guidelines", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Community Guidelines", voter: "calmpanda", direction: 1 },

    VoteSeed { post_title: "Site Updates — June 2025", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Site Updates — June 2025", voter: "alice", direction: 1 },
    VoteSeed { post_title: "Site Updates — June 2025", voter: "milddeer", direction: 1 },

    VoteSeed { post_title: "Hi everyone, I'm new here!", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "Hi everyone, I'm new here!", voter: "alice", direction: 1 },
    VoteSeed { post_title: "Hi everyone, I'm new here!", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Hi everyone, I'm new here!", voter: "milddeer", direction: 1 },
    VoteSeed { post_title: "Hi everyone, I'm new here!", voter: "gentleotter", direction: 1 },

    VoteSeed { post_title: "Long-time lurker, finally posting", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "Long-time lurker, finally posting", voter: "alice", direction: 1 },
    VoteSeed { post_title: "Long-time lurker, finally posting", voter: "noblelion", direction: 1 },

    VoteSeed { post_title: "Greetings from the UK burrow!", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Greetings from the UK burrow!", voter: "eagerhare", direction: 1 },

    VoteSeed { post_title: "What's everyone snacking on today?", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "What's everyone snacking on today?", voter: "bob", direction: 1 },
    VoteSeed { post_title: "What's everyone snacking on today?", voter: "noblelion", direction: 1 },
    VoteSeed { post_title: "What's everyone snacking on today?", voter: "wildmoose", direction: 1 },
    VoteSeed { post_title: "What's everyone snacking on today?", voter: "eagerhare", direction: 1 },

    VoteSeed { post_title: "Forum rules and guidelines", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "Forum rules and guidelines", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Forum rules and guidelines", voter: "sillygoat", direction: -1 },

    VoteSeed { post_title: "Hot take: mornings are better than nights", voter: "calmpanda", direction: 1 },
    VoteSeed { post_title: "Hot take: mornings are better than nights", voter: "wildmoose", direction: -1 },
    VoteSeed { post_title: "Hot take: mornings are better than nights", voter: "cleverfox", direction: 1 },
    VoteSeed { post_title: "Hot take: mornings are better than nights", voter: "jollyfrog", direction: -1 },

    VoteSeed { post_title: "What's your favorite mouse pun?", voter: "calmpanda", direction: 1 },
    VoteSeed { post_title: "What's your favorite mouse pun?", voter: "sillygoat", direction: 1 },
    VoteSeed { post_title: "What's your favorite mouse pun?", voter: "bob", direction: 1 },
    VoteSeed { post_title: "What's your favorite mouse pun?", voter: "jollyfrog", direction: 1 },
    VoteSeed { post_title: "What's your favorite mouse pun?", voter: "cleverfox", direction: 1 },

    VoteSeed { post_title: "Should we have a community event?", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "Should we have a community event?", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Should we have a community event?", voter: "swiftowl", direction: 1 },
    VoteSeed { post_title: "Should we have a community event?", voter: "bravebadger", direction: 1 },

    VoteSeed { post_title: "Best cheddar I've ever nibbled", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "Best cheddar I've ever nibbled", voter: "alice", direction: 1 },
    VoteSeed { post_title: "Best cheddar I've ever nibbled", voter: "noblelion", direction: 1 },
    VoteSeed { post_title: "Best cheddar I've ever nibbled", voter: "eagerhare", direction: 1 },
    VoteSeed { post_title: "Best cheddar I've ever nibbled", voter: "calmpanda", direction: 1 },
    VoteSeed { post_title: "Best cheddar I've ever nibbled", voter: "deepmole", direction: 1 },

    VoteSeed { post_title: "Gouda vs Edam: the ultimate showdown", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Gouda vs Edam: the ultimate showdown", voter: "alice", direction: -1 },
    VoteSeed { post_title: "Gouda vs Edam: the ultimate showdown", voter: "noblelion", direction: 1 },
    VoteSeed { post_title: "Gouda vs Edam: the ultimate showdown", voter: "calmpanda", direction: -1 },
    VoteSeed { post_title: "Gouda vs Edam: the ultimate showdown", voter: "eagerhare", direction: 1 },

    VoteSeed { post_title: "Made my first homemade camembert", voter: "calmpanda", direction: 1 },
    VoteSeed { post_title: "Made my first homemade camembert", voter: "deepmole", direction: 1 },
    VoteSeed { post_title: "Made my first homemade camembert", voter: "eagerhare", direction: 1 },
    VoteSeed { post_title: "Made my first homemade camembert", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Made my first homemade camembert", voter: "alice", direction: 1 },
    VoteSeed { post_title: "Made my first homemade camembert", voter: "mouse", direction: 1 },

    VoteSeed { post_title: "Blue cheese: love it or hate it?", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "Blue cheese: love it or hate it?", voter: "noblelion", direction: 1 },
    VoteSeed { post_title: "Blue cheese: love it or hate it?", voter: "gentleotter", direction: -1 },
    VoteSeed { post_title: "Blue cheese: love it or hate it?", voter: "oddotter", direction: 1 },
    VoteSeed { post_title: "Blue cheese: love it or hate it?", voter: "wildmoose", direction: -1 },

    VoteSeed { post_title: "Planning a cheese tasting party", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Planning a cheese tasting party", voter: "noblelion", direction: 1 },
    VoteSeed { post_title: "Planning a cheese tasting party", voter: "jollyfrog", direction: 1 },
    VoteSeed { post_title: "Planning a cheese tasting party", voter: "eagerhare", direction: 1 },

    VoteSeed { post_title: "Building my first mechanical keyboard", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "Building my first mechanical keyboard", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Building my first mechanical keyboard", voter: "bravebadger", direction: 1 },
    VoteSeed { post_title: "Building my first mechanical keyboard", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "Building my first mechanical keyboard", voter: "swiftowl", direction: 1 },
    VoteSeed { post_title: "Building my first mechanical keyboard", voter: "wildmoose", direction: 1 },

    VoteSeed { post_title: "Rust tip: using Result with axum", voter: "alice", direction: 1 },
    VoteSeed { post_title: "Rust tip: using Result with axum", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Rust tip: using Result with axum", voter: "bravebadger", direction: 1 },
    VoteSeed { post_title: "Rust tip: using Result with axum", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "Rust tip: using Result with axum", voter: "cleverfox", direction: 1 },

    VoteSeed { post_title: "My SFF PC build for 2025", voter: "alice", direction: 1 },
    VoteSeed { post_title: "My SFF PC build for 2025", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "My SFF PC build for 2025", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "My SFF PC build for 2025", voter: "bravebadger", direction: 1 },

    VoteSeed { post_title: "NixOS: first impressions after a month", voter: "tinybat", direction: 1 },
    VoteSeed { post_title: "NixOS: first impressions after a month", voter: "bravebadger", direction: 1 },
    VoteSeed { post_title: "NixOS: first impressions after a month", voter: "cleverfox", direction: 1 },

    VoteSeed { post_title: "What IDE do you use and why?", voter: "cleverfox", direction: 1 },
    VoteSeed { post_title: "What IDE do you use and why?", voter: "swiftowl", direction: 1 },
    VoteSeed { post_title: "What IDE do you use and why?", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "What IDE do you use and why?", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "What IDE do you use and why?", voter: "alice", direction: 1 },

    VoteSeed { post_title: "My latest watercolor: sunset over the wheat field", voter: "mouse", direction: 1 },
    VoteSeed { post_title: "My latest watercolor: sunset over the wheat field", voter: "fancyrat", direction: 1 },
    VoteSeed { post_title: "My latest watercolor: sunset over the wheat field", voter: "royalcat", direction: 1 },
    VoteSeed { post_title: "My latest watercolor: sunset over the wheat field", voter: "alice", direction: 1 },

    VoteSeed { post_title: "Drawn with cheese: edible art thread", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Drawn with cheese: edible art thread", voter: "sillygoat", direction: 1 },
    VoteSeed { post_title: "Drawn with cheese: edible art thread", voter: "jollyfrog", direction: 1 },

    VoteSeed { post_title: "Digital painting of the burrow at dawn", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Digital painting of the burrow at dawn", voter: "royalcat", direction: 1 },
    VoteSeed { post_title: "Digital painting of the burrow at dawn", voter: "mouse", direction: 1 },

    VoteSeed { post_title: "Polymer clay mouse sculpture", voter: "gentleotter", direction: 1 },
    VoteSeed { post_title: "Polymer clay mouse sculpture", voter: "fancyrat", direction: 1 },
    VoteSeed { post_title: "Polymer clay mouse sculpture", voter: "alice", direction: 1 },
    VoteSeed { post_title: "Polymer clay mouse sculpture", voter: "bob", direction: 1 },

    VoteSeed { post_title: "Cheese Heist speedrun world record broken!", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Cheese Heist speedrun world record broken!", voter: "alice", direction: -1 },
    VoteSeed { post_title: "Cheese Heist speedrun world record broken!", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "Cheese Heist speedrun world record broken!", voter: "swiftowl", direction: 1 },
    VoteSeed { post_title: "Cheese Heist speedrun world record broken!", voter: "wildmoose", direction: 1 },

    VoteSeed { post_title: "What are you playing this weekend?", voter: "swiftowl", direction: 1 },
    VoteSeed { post_title: "What are you playing this weekend?", voter: "gentleotter", direction: 1 },
    VoteSeed { post_title: "What are you playing this weekend?", voter: "bob", direction: 1 },

    VoteSeed { post_title: "Balatro is consuming my life", voter: "wildmoose", direction: 1 },
    VoteSeed { post_title: "Balatro is consuming my life", voter: "eagerhare", direction: 1 },
    VoteSeed { post_title: "Balatro is consuming my life", voter: "swiftowl", direction: 1 },
    VoteSeed { post_title: "Balatro is consuming my life", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "Balatro is consuming my life", voter: "mouse", direction: 1 },

    VoteSeed { post_title: "Steam Deck vs ROG Ally: which one?", voter: "bravebadger", direction: 1 },
    VoteSeed { post_title: "Steam Deck vs ROG Ally: which one?", voter: "wildmoose", direction: 1 },
    VoteSeed { post_title: "Steam Deck vs ROG Ally: which one?", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "Steam Deck vs ROG Ally: which one?", voter: "mouse", direction: 1 },

    VoteSeed { post_title: "My retro gaming corner setup", voter: "keenwolf", direction: 1 },
    VoteSeed { post_title: "My retro gaming corner setup", voter: "sillygoat", direction: 1 },
    VoteSeed { post_title: "My retro gaming corner setup", voter: "wildmoose", direction: 1 },

    VoteSeed { post_title: "Just finished 'The Mouse and the Motorcycle'", voter: "calmpanda", direction: 1 },
    VoteSeed { post_title: "Just finished 'The Mouse and the Motorcycle'", voter: "bob", direction: 1 },

    VoteSeed { post_title: "Cozy fantasy recommendations?", voter: "cleverfox", direction: 1 },
    VoteSeed { post_title: "Cozy fantasy recommendations?", voter: "happybunny", direction: 1 },
    VoteSeed { post_title: "Cozy fantasy recommendations?", voter: "calmpanda", direction: 1 },
    VoteSeed { post_title: "Cozy fantasy recommendations?", voter: "milddeer", direction: 1 },

    VoteSeed { post_title: "Dune re-read — new insights as an adult", voter: "proudbear", direction: 1 },
    VoteSeed { post_title: "Dune re-read — new insights as an adult", voter: "tinybat", direction: 1 },
    VoteSeed { post_title: "Dune re-read — new insights as an adult", voter: "cleverfox", direction: 1 },
    VoteSeed { post_title: "Dune re-read — new insights as an adult", voter: "milddeer", direction: 1 },
    VoteSeed { post_title: "Dune re-read — new insights as an adult", voter: "mouse", direction: 1 },

    VoteSeed { post_title: "Building a cozy reading corner", voter: "calmpanda", direction: 1 },
    VoteSeed { post_title: "Building a cozy reading corner", voter: "happybunny", direction: 1 },
    VoteSeed { post_title: "Building a cozy reading corner", voter: "alice", direction: 1 },

    VoteSeed { post_title: "How's the weather in your burrow?", voter: "jollyfrog", direction: 1 },
    VoteSeed { post_title: "How's the weather in your burrow?", voter: "milddeer", direction: 1 },
    VoteSeed { post_title: "How's the weather in your burrow?", voter: "eagerhare", direction: 1 },

    VoteSeed { post_title: "Pet tax thread: show us your pets!", voter: "deepmole", direction: 1 },
    VoteSeed { post_title: "Pet tax thread: show us your pets!", voter: "eagerhare", direction: 1 },
    VoteSeed { post_title: "Pet tax thread: show us your pets!", voter: "gentleotter", direction: 1 },
    VoteSeed { post_title: "Pet tax thread: show us your pets!", voter: "fancyrat", direction: 1 },

    VoteSeed { post_title: "What music are you listening to?", voter: "calmpanda", direction: 1 },
    VoteSeed { post_title: "What music are you listening to?", voter: "cleverfox", direction: 1 },
    VoteSeed { post_title: "What music are you listening to?", voter: "oddotter", direction: 1 },

    VoteSeed { post_title: "Sourdough starter day 7 — it's alive!", voter: "bob", direction: 1 },
    VoteSeed { post_title: "Sourdough starter day 7 — it's alive!", voter: "milddeer", direction: 1 },
    VoteSeed { post_title: "Sourdough starter day 7 — it's alive!", voter: "oddotter", direction: 1 },
    VoteSeed { post_title: "Sourdough starter day 7 — it's alive!", voter: "noblelion", direction: 1 },
    VoteSeed { post_title: "Sourdough starter day 7 — it's alive!", voter: "mouse", direction: 1 },

    VoteSeed { post_title: "One-pot pasta that changed my life", voter: "noblelion", direction: 1 },
    VoteSeed { post_title: "One-pot pasta that changed my life", voter: "eagerhare", direction: 1 },
    VoteSeed { post_title: "One-pot pasta that changed my life", voter: "milddeer", direction: 1 },
    VoteSeed { post_title: "One-pot pasta that changed my life", voter: "alice", direction: 1 },

    VoteSeed { post_title: "My fermentation station setup", voter: "noblelion", direction: 1 },
    VoteSeed { post_title: "My fermentation station setup", voter: "milddeer", direction: 1 },
    VoteSeed { post_title: "My fermentation station setup", voter: "bob", direction: 1 },
    VoteSeed { post_title: "My fermentation station setup", voter: "eagerhare", direction: 1 },
];

// ── Comment votes ─────────────────────────────────────────────────────

pub static COMMENT_VOTES: &[CmtVoteSeed] = &[
    CmtVoteSeed { comment_idx: 0, voter: "swiftowl", direction: 1 },
    CmtVoteSeed { comment_idx: 0, voter: "bravebadger", direction: 1 },
    CmtVoteSeed { comment_idx: 1, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 1, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 2, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 2, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 3, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 3, voter: "noblelion", direction: 1 },
    CmtVoteSeed { comment_idx: 4, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 4, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 5, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 5, voter: "calmpanda", direction: 1 },
    CmtVoteSeed { comment_idx: 6, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 6, voter: "noblelion", direction: 1 },
    CmtVoteSeed { comment_idx: 7, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 7, voter: "eagerhare", direction: 1 },
    CmtVoteSeed { comment_idx: 8, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 8, voter: "bravebadger", direction: 1 },
    CmtVoteSeed { comment_idx: 9, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 9, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 10, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 10, voter: "bravebadger", direction: 1 },
    CmtVoteSeed { comment_idx: 10, voter: "keenwolf", direction: 1 },
    CmtVoteSeed { comment_idx: 11, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 11, voter: "fancyrat", direction: 1 },
    CmtVoteSeed { comment_idx: 11, voter: "royalcat", direction: 1 },
    CmtVoteSeed { comment_idx: 12, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 12, voter: "sillygoat", direction: 1 },
    CmtVoteSeed { comment_idx: 13, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 13, voter: "keenwolf", direction: 1 },
    CmtVoteSeed { comment_idx: 13, voter: "swiftowl", direction: 1 },
    CmtVoteSeed { comment_idx: 14, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 14, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 14, voter: "calmpanda", direction: 1 },

    // New comment votes
    CmtVoteSeed { comment_idx: 15, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 15, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 16, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 16, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 22, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 22, voter: "swiftowl", direction: 1 },
    CmtVoteSeed { comment_idx: 23, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 26, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 27, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 27, voter: "jollyfrog", direction: 1 },
    CmtVoteSeed { comment_idx: 28, voter: "noblelion", direction: 1 },
    CmtVoteSeed { comment_idx: 29, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 29, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 30, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 30, voter: "eagerhare", direction: 1 },
    CmtVoteSeed { comment_idx: 31, voter: "wildmoose", direction: 1 },
    CmtVoteSeed { comment_idx: 32, voter: "calmpanda", direction: 1 },
    CmtVoteSeed { comment_idx: 33, voter: "calmpanda", direction: 1 },
    CmtVoteSeed { comment_idx: 34, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 35, voter: "cleverfox", direction: 1 },
    CmtVoteSeed { comment_idx: 36, voter: "eagerhare", direction: 1 },
    CmtVoteSeed { comment_idx: 37, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 37, voter: "cleverfox", direction: 1 },
    CmtVoteSeed { comment_idx: 38, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 39, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 39, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 40, voter: "swiftowl", direction: 1 },
    CmtVoteSeed { comment_idx: 40, voter: "bravebadger", direction: 1 },
    CmtVoteSeed { comment_idx: 41, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 41, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 42, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 43, voter: "noblelion", direction: 1 },
    CmtVoteSeed { comment_idx: 44, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 44, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 45, voter: "eagerhare", direction: 1 },
    CmtVoteSeed { comment_idx: 45, voter: "calmpanda", direction: 1 },
    CmtVoteSeed { comment_idx: 46, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 51, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 51, voter: "noblelion", direction: 1 },
    CmtVoteSeed { comment_idx: 52, voter: "keenwolf", direction: 1 },
    CmtVoteSeed { comment_idx: 52, voter: "oddotter", direction: 1 },
    CmtVoteSeed { comment_idx: 53, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 53, voter: "eagerhare", direction: 1 },
    CmtVoteSeed { comment_idx: 55, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 55, voter: "bravebadger", direction: 1 },
    CmtVoteSeed { comment_idx: 56, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 57, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 58, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 59, voter: "bravebadger", direction: 1 },
    CmtVoteSeed { comment_idx: 62, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 62, voter: "fancyrat", direction: 1 },
    CmtVoteSeed { comment_idx: 65, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 65, voter: "royalcat", direction: 1 },
    CmtVoteSeed { comment_idx: 66, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 69, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 69, voter: "swiftowl", direction: 1 },
    CmtVoteSeed { comment_idx: 70, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 72, voter: "swiftowl", direction: 1 },
    CmtVoteSeed { comment_idx: 72, voter: "wildmoose", direction: 1 },
    CmtVoteSeed { comment_idx: 73, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 73, voter: "swiftowl", direction: 1 },
    CmtVoteSeed { comment_idx: 74, voter: "eagerhare", direction: 1 },
    CmtVoteSeed { comment_idx: 75, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 78, voter: "alice", direction: 1 },
    CmtVoteSeed { comment_idx: 78, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 79, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 79, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 80, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 83, voter: "cleverfox", direction: 1 },
    CmtVoteSeed { comment_idx: 84, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 85, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 86, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 87, voter: "gentleotter", direction: 1 },
    CmtVoteSeed { comment_idx: 88, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 89, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 89, voter: "milddeer", direction: 1 },
    CmtVoteSeed { comment_idx: 90, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 91, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 92, voter: "noblelion", direction: 1 },
    CmtVoteSeed { comment_idx: 92, voter: "eagerhare", direction: 1 },
    CmtVoteSeed { comment_idx: 93, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 93, voter: "noblelion", direction: 1 },
    CmtVoteSeed { comment_idx: 94, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 96, voter: "mouse", direction: 1 },
    CmtVoteSeed { comment_idx: 97, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 97, voter: "milddeer", direction: 1 },
    CmtVoteSeed { comment_idx: 97, voter: "noblelion", direction: 1 },
    CmtVoteSeed { comment_idx: 98, voter: "bob", direction: 1 },
    CmtVoteSeed { comment_idx: 98, voter: "milddeer", direction: 1 },
    CmtVoteSeed { comment_idx: 99, voter: "bob", direction: 1 },
];

// ── Follows (feed db: follower → board) ──────────────────────────────

pub static FOLLOWS: &[FollowSeed] = &[
    FollowSeed { follower: "mouse", board_name: "announcements" },
    FollowSeed { follower: "mouse", board_name: "general" },
    FollowSeed { follower: "mouse", board_name: "tech" },
    FollowSeed { follower: "mouse", board_name: "gaming" },
    FollowSeed { follower: "mouse", board_name: "books" },
    FollowSeed { follower: "alice", board_name: "cheese" },
    FollowSeed { follower: "alice", board_name: "art" },
    FollowSeed { follower: "alice", board_name: "books" },
    FollowSeed { follower: "alice", board_name: "tech" },
    FollowSeed { follower: "alice", board_name: "general" },
    FollowSeed { follower: "bob", board_name: "gaming" },
    FollowSeed { follower: "bob", board_name: "cheese" },
    FollowSeed { follower: "bob", board_name: "cooking" },
    FollowSeed { follower: "bob", board_name: "general" },
    FollowSeed { follower: "swiftowl", board_name: "gaming" },
    FollowSeed { follower: "swiftowl", board_name: "tech" },
    FollowSeed { follower: "swiftowl", board_name: "books" },
    FollowSeed { follower: "bravebadger", board_name: "tech" },
    FollowSeed { follower: "bravebadger", board_name: "gaming" },
    FollowSeed { follower: "bravebadger", board_name: "general" },
    FollowSeed { follower: "calmpanda", board_name: "books" },
    FollowSeed { follower: "calmpanda", board_name: "off-topic" },
    FollowSeed { follower: "calmpanda", board_name: "cheese" },
    FollowSeed { follower: "cleverfox", board_name: "tech" },
    FollowSeed { follower: "cleverfox", board_name: "books" },
    FollowSeed { follower: "cleverfox", board_name: "general" },
    FollowSeed { follower: "eagerhare", board_name: "general" },
    FollowSeed { follower: "eagerhare", board_name: "cheese" },
    FollowSeed { follower: "eagerhare", board_name: "cooking" },
    FollowSeed { follower: "keenwolf", board_name: "tech" },
    FollowSeed { follower: "keenwolf", board_name: "gaming" },
    FollowSeed { follower: "noblelion", board_name: "cheese" },
    FollowSeed { follower: "noblelion", board_name: "cooking" },
    FollowSeed { follower: "milddeer", board_name: "books" },
    FollowSeed { follower: "milddeer", board_name: "cooking" },
    FollowSeed { follower: "milddeer", board_name: "general" },
    FollowSeed { follower: "wildmoose", board_name: "gaming" },
    FollowSeed { follower: "wildmoose", board_name: "off-topic" },
    FollowSeed { follower: "fancyrat", board_name: "art" },
    FollowSeed { follower: "fancyrat", board_name: "general" },
];

// ── Feed preferences ─────────────────────────────────────────────────

pub static PREFERENCES: &[PrefSeed] = &[
    PrefSeed { user: "mouse", sort: "hot", muted_boards: &[] },
    PrefSeed { user: "alice", sort: "new", muted_boards: &[] },
    PrefSeed { user: "bob", sort: "top", muted_boards: &["art"] },
    PrefSeed { user: "swiftowl", sort: "new", muted_boards: &[] },
    PrefSeed { user: "bravebadger", sort: "hot", muted_boards: &["off-topic"] },
    PrefSeed { user: "calmpanda", sort: "hot", muted_boards: &[] },
    PrefSeed { user: "cleverfox", sort: "new", muted_boards: &[] },
    PrefSeed { user: "eagerhare", sort: "top", muted_boards: &[] },
    PrefSeed { user: "keenwolf", sort: "hot", muted_boards: &["announcements"] },
    PrefSeed { user: "noblelion", sort: "new", muted_boards: &[] },
    PrefSeed { user: "milddeer", sort: "top", muted_boards: &[] },
    PrefSeed { user: "wildmoose", sort: "hot", muted_boards: &[] },
    PrefSeed { user: "fancyrat", sort: "new", muted_boards: &["gaming"] },
    PrefSeed { user: "jollyfrog", sort: "top", muted_boards: &["tech"] },
    PrefSeed { user: "gentleotter", sort: "hot", muted_boards: &[] },
];

// ── Friend requests (social db) ──────────────────────────────────────

pub static FRIEND_REQUESTS: &[FriendRequestSeed] = &[
    FriendRequestSeed { sender: "mouse", receiver: "alice", status: "pending" },
    FriendRequestSeed { sender: "alice", receiver: "bob", status: "pending" },
    FriendRequestSeed { sender: "bob", receiver: "mouse", status: "accepted" },
    FriendRequestSeed { sender: "alice", receiver: "swiftowl", status: "accepted" },
    FriendRequestSeed { sender: "eagerhare", receiver: "alice", status: "pending" },
    FriendRequestSeed { sender: "bravebadger", receiver: "keenwolf", status: "accepted" },
    FriendRequestSeed { sender: "cleverfox", receiver: "mouse", status: "accepted" },
    FriendRequestSeed { sender: "noblelion", receiver: "bob", status: "pending" },
    FriendRequestSeed { sender: "milddeer", receiver: "calmpanda", status: "accepted" },
    FriendRequestSeed { sender: "wildmoose", receiver: "keenwolf", status: "pending" },
    FriendRequestSeed { sender: "fancyrat", receiver: "royalcat", status: "pending" },
    FriendRequestSeed { sender: "bob", receiver: "noblelion", status: "accepted" },
    FriendRequestSeed { sender: "jollyfrog", receiver: "eagerhare", status: "pending" },
    FriendRequestSeed { sender: "gentleotter", receiver: "milddeer", status: "pending" },
    FriendRequestSeed { sender: "swiftowl", receiver: "wildmoose", status: "accepted" },
];

// ── Friendships (social db) ──────────────────────────────────────────

pub static FRIENDSHIPS: &[FriendshipSeed] = &[
    FriendshipSeed { user: "bob", friend: "mouse" },
    FriendshipSeed { user: "mouse", friend: "bob" },
    FriendshipSeed { user: "alice", friend: "swiftowl" },
    FriendshipSeed { user: "swiftowl", friend: "alice" },
    FriendshipSeed { user: "bravebadger", friend: "keenwolf" },
    FriendshipSeed { user: "keenwolf", friend: "bravebadger" },
    FriendshipSeed { user: "cleverfox", friend: "mouse" },
    FriendshipSeed { user: "mouse", friend: "cleverfox" },
    FriendshipSeed { user: "milddeer", friend: "calmpanda" },
    FriendshipSeed { user: "calmpanda", friend: "milddeer" },
    FriendshipSeed { user: "bob", friend: "noblelion" },
    FriendshipSeed { user: "noblelion", friend: "bob" },
    FriendshipSeed { user: "swiftowl", friend: "wildmoose" },
    FriendshipSeed { user: "wildmoose", friend: "swiftowl" },
];

// ── Reports (moderation db) ──────────────────────────────────────────

pub static REPORTS: &[ReportSeed] = &[
    ReportSeed {
        reporter: "mouse",
        target_type: "post",
        target_post_title: "Best cheddar I've ever nibbled",
        target_comment_idx: None,
        reason: "Contains what looks like spam advertising for a cheese vendor",
        resolved: false,
    },
    ReportSeed {
        reporter: "alice",
        target_type: "comment",
        target_post_title: "Gouda vs Edam: the ultimate showdown",
        target_comment_idx: Some(7),
        reason: "This comment uses harassing language about cheese preferences",
        resolved: false,
    },
    ReportSeed {
        reporter: "bob",
        target_type: "post",
        target_post_title: "Drawn with cheese: edible art thread",
        target_comment_idx: None,
        reason: "Off-topic — should this be in the art board?",
        resolved: true,
    },
    ReportSeed {
        reporter: "alice",
        target_type: "comment",
        target_post_title: "Forum rules and guidelines",
        target_comment_idx: Some(31),
        reason: "Responding to the rules with a sarcastic comment about stealing cheese",
        resolved: false,
    },
    ReportSeed {
        reporter: "calmpanda",
        target_type: "comment",
        target_post_title: "Hot take: mornings are better than nights",
        target_comment_idx: Some(33),
        reason: "Unnecessarily aggressive tone in a discussion about preferences",
        resolved: false,
    },
    ReportSeed {
        reporter: "bob",
        target_type: "post",
        target_post_title: "Blue cheese: love it or hate it?",
        target_comment_idx: None,
        reason: "The post title encourages polarizing and potentially hostile debate",
        resolved: true,
    },
    ReportSeed {
        reporter: "eagerhare",
        target_type: "comment",
        target_post_title: "Steam Deck vs ROG Ally: which one?",
        target_comment_idx: Some(75),
        reason: "Comment contains console war flame-baiting",
        resolved: false,
    },
    ReportSeed {
        reporter: "mouse",
        target_type: "comment",
        target_post_title: "How's the weather in your burrow?",
        target_comment_idx: Some(84),
        reason: "User is posting the same content across multiple threads (spam)",
        resolved: true,
    },
];

// ── Moderation actions (moderation db) ────────────────────────────────

pub static MOD_ACTIONS: &[ModActionSeed] = &[
    ModActionSeed {
        moderator: "mouse",
        action_type: "warn",
        target_post_title: "Drawn with cheese: edible art thread",
        reason: "Post moved to correct board — please keep topics in their designated boards",
    },
    ModActionSeed {
        moderator: "bob",
        action_type: "lock",
        target_post_title: "Blue cheese: love it or hate it?",
        reason: "Thread locked due to heated debate — take a break and revisit with cooler heads",
    },
    ModActionSeed {
        moderator: "mouse",
        action_type: "warn",
        target_post_title: "How's the weather in your burrow?",
        reason: "Repeated cross-posting of the same content. Please keep duplicate posts to a minimum.",
    },
    ModActionSeed {
        moderator: "alice",
        action_type: "delete",
        target_post_title: "Hot take: mornings are better than nights",
        reason: "Comment removed for violating community guideline 1 (be kind)",
    },
    ModActionSeed {
        moderator: "mouse",
        action_type: "warn",
        target_post_title: "Steam Deck vs ROG Ally: which one?",
        reason: "Please keep discussion civil — platform preference discussions should not include personal attacks",
    },
];
