//! One course. Three modules. Pattern cards — not a diagnosis.

#[derive(Clone, Copy, Debug)]
pub struct Street {
    pub id: &'static str,
    pub tag: &'static str,
    pub title: &'static str,
    pub scene: &'static str,
    pub teach: &'static str,
    pub why: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Quest {
    pub id: &'static str,
    pub tag: &'static str,
    pub name: &'static str,
    pub goal: &'static str,
    pub streets: &'static [Street],
}

pub static QUESTS: &[Quest] = &[OVERT, COVERT, CORP];

pub fn street_count() -> usize {
    QUESTS.iter().map(|q| q.streets.len()).sum()
}

pub fn find_street(qid: &str, sid: &str) -> Option<(usize, usize, &'static Quest, &'static Street)> {
    for (qi, q) in QUESTS.iter().enumerate() {
        if q.id != qid {
            continue;
        }
        for (si, s) in q.streets.iter().enumerate() {
            if s.id == sid {
                return Some((qi, si, q, s));
            }
        }
    }
    None
}

pub fn next_street(qi: usize, si: usize) -> Option<(&'static Quest, &'static Street)> {
    let q = QUESTS.get(qi)?;
    if let Some(s) = q.streets.get(si + 1) {
        return Some((q, s));
    }
    let q2 = QUESTS.get(qi + 1)?;
    Some((q2, q2.streets.first()?))
}

const OVERT: Quest = Quest {
    id: "overt",
    tag: "OVERT",
    name: "Overt narcissist",
    goal: "Loud. Centre of the room. Status over substance.",
    streets: &[
        Street {
            id: "credit",
            tag: "1",
            title: "Credit theft",
            scene: "They demo your work as something they drove. The room applauds the wrong name. A shoulder-clap after is not credit.",
            teach: "They need the audience more than the work. Status moves to whoever talks last in the room that matters.",
            why: "Keep the artifact dated. This is a transfer of whose name leadership remembers — not presenting.",
        },
        Street {
            id: "stage",
            tag: "2",
            title: "Holding the room",
            scene: "Meetings become monologues. They talk over you. The decision is whatever they said last. Your sentence never lands.",
            teach: "The point is not the decision. The point is that they were speaking when it happened.",
            why: "Put the risk in writing. Do not compete for air they already own.",
        },
        Street {
            id: "rage",
            tag: "3",
            title: "Narcissistic injury",
            scene: "You correct a fact in public. Their face changes. After, you are 'not a team player.' The fact was right. The punishment is social.",
            teach: "Challenge, even mild, is an injury to the self they perform.",
            why: "Correct in writing, not as theatre. They remember the no, not the number.",
        },
        Street {
            id: "rules",
            tag: "4",
            title: "Special rules",
            scene: "Process applies to you. Exceptions apply to them. When you ask, you are being bureaucratic.",
            teach: "Specialness is the operating system. Rules are a tool applied downward.",
            why: "Document the exception. Do not argue fairness in a room they already won.",
        },
        Street {
            id: "charm",
            tag: "5",
            title: "Upward charm",
            scene: "Leadership loves them. Peers go quiet when they leave. Juniors stop asking questions. Charm is spent upward, never down.",
            teach: "Warmth is directional. Believe the people below them, not the exec thread.",
            why: "The canary is the newest person who went silent.",
        },
        Street {
            id: "shame",
            tag: "6",
            title: "Public shame",
            scene: "A private performance issue is named on the team call. The audience is the point. After, they were 'just being direct.'",
            teach: "Public critique of a person, not a diff, is a status play. Humiliation is the product.",
            why: "Feedback about a person belongs behind a door. Feedback about a diff belongs on the diff.",
        },
        Street {
            id: "legend",
            tag: "7",
            title: "Rewriting history",
            scene: "The team 'was nothing until they arrived.' Your history is a footnote or a problem they fixed. People who remember otherwise become inconvenient.",
            teach: "They rewrite the past so they are the founder of every win.",
            why: "Keep dates, old decks, old commits. Legend cannot survive a timeline.",
        },
    ],
};

const COVERT: Quest = Quest {
    id: "covert",
    tag: "COVERT",
    name: "Covert narcissist",
    goal: "Quiet. Wounded. The knife is a sigh.",
    streets: &[
        Street {
            id: "victim",
            tag: "1",
            title: "Victim play",
            scene: "You ask them to choose. They look tired: they care too much to decide. Sympathy arrives. The work still sits on you.",
            teach: "Victimhood collects care and dodges ownership at the same time.",
            why: "Stay on the ticket. Who owns this by Friday? Do not comfort the person who just handed you their job.",
        },
        Street {
            id: "silent",
            tag: "2",
            title: "Silent treatment",
            scene: "You disagreed politely. They stop answering. The project stalls. They 'needed space.' The space was the punishment.",
            teach: "Silence makes you chase, apologize, and rewrite what you said.",
            why: "Do not chase. Put the ask in writing with a date. Escalate the work, not the feeling.",
        },
        Street {
            id: "stab",
            tag: "3",
            title: "Backhanded cut",
            scene: "Review comment: 'interesting choice.' No suggestion. You spend the afternoon wondering. That was the payload.",
            teach: "A hint without a request is a status cut. You do the self-doubt. They keep deniability.",
            why: "Ask what should change. If they will not say, it is weather, not a review.",
        },
        Street {
            id: "smear",
            tag: "4",
            title: "Quiet smear",
            scene: "They are 'worried about you' to other people. You hear it third-hand. Nobody will quote them. The weather around you is worse.",
            teach: "The smear is caring on the surface. It never has to face you, so it never has to be true.",
            why: "Ask who said it, in writing. Kitchen weather is not a record until you make one.",
        },
        Street {
            id: "helpless",
            tag: "5",
            title: "Helpless dump",
            scene: "'You're so much faster than me.' Last-minute work, not on the board. The compliment is wrapping so you cannot call it a dump.",
            teach: "Learned helplessness transfers labour. Flattery moves the pile without a name.",
            why: "Answer in the thread: scope, date, what slides if not. Compliments are not assignments.",
        },
        Street {
            id: "moral",
            tag: "6",
            title: "Weaponized honesty",
            scene: "They undercut you, then claim honesty. If you flinch you are too sensitive. If you don't, they do it again.",
            teach: "Honesty without kindness or specificity is a license to cut.",
            why: "Ask for the concrete change. If there is none, it was not feedback.",
        },
        Street {
            id: "withhold",
            tag: "7",
            title: "Withholding",
            scene: "The goal changed. They thought you knew. You look unprepared. They look concerned.",
            teach: "Withholding keeps them needed and you wrong. Information is metered.",
            why: "Decisions in the doc. Ask when it was decided and where it is written.",
        },
    ],
};

const CORP: Quest = Quest {
    id: "corp",
    tag: "CORP",
    name: "Corporate psychopath",
    goal: "Charm, use, discard. No heat. The damage is the plan.",
    streets: &[
        Street {
            id: "mask",
            tag: "1",
            title: "The mask",
            scene: "First they are easy: jokes, access, 'we're a team.' Then you are a function. Then you are in the way. Nobody else saw the switch.",
            teach: "The warmth was a tool. The cold is the real setting.",
            why: "Watch what they do when you are no longer useful, not how they greeted you.",
        },
        Street {
            id: "lie",
            tag: "2",
            title: "Shameless lie",
            scene: "They promised something. They deny the conversation. No shame, no repair. You look like you imagined it.",
            teach: "Lies are cheap because they feel nothing when caught. Your memory becomes the problem.",
            why: "Follow up in writing the same day. If they will not confirm, you already have the tell.",
        },
        Street {
            id: "tool",
            tag: "3",
            title: "Using people",
            scene: "They used your intro, your deck, your weekend. The win is theirs. You are not mentioned. When you ask, they look bored.",
            teach: "People are instruments. Once the job is done, there is no leftover loyalty.",
            why: "Do not lend your name without a written role. They will not become grateful later.",
        },
        Street {
            id: "split",
            tag: "4",
            title: "Splitting the team",
            scene: "They tell you the other person is the problem. They tell the other person you are. Both of you work late. They stay clean.",
            teach: "Divide and conquer keeps witnesses from comparing notes.",
            why: "Compare notes with the artifact, not the feeling. Isolation is the design.",
        },
        Street {
            id: "risk",
            tag: "5",
            title: "Risk dump",
            scene: "They push a thin launch. If it works, they demo it. If it fails, your name is on the blast radius. They were just asking questions.",
            teach: "Reward up, risk down. They assign outcomes. They do not share them.",
            why: "Get the go-live in writing: who decided, what was accepted.",
        },
        Street {
            id: "empty",
            tag: "6",
            title: "No empathy",
            scene: "Someone is in pieces. They notice the delay, not the person. A joke, a shrug, next agenda item.",
            teach: "Lack of empathy is the absence of a brake. Harm does not register, so it repeats.",
            why: "Do not wait for a conscience. Protect the person. Move work and witnesses, not appeals.",
        },
        Street {
            id: "exit",
            tag: "7",
            title: "Leave",
            scene: "You have dates, diffs, one ally. The pattern is still the pattern. Charm still works upstairs.",
            teach: "You cannot treat a person who feels no cost. Staying to prove toughness is how people get sick.",
            why: "Document, then decide with a clear head. Training names the pattern. It cannot make the pattern kinder.",
        },
    ],
};
