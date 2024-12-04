#[derive(PartialEq, Eq)]
pub enum SupportLevel {
    PERFECT,
    GREAT,
    GOOD,
    BAD,
    FAIL,
}

#[derive(PartialEq, Eq)]
pub enum TranslationLayer {
    LATX,
    BOX64,
    LATA,
}

#[derive(PartialEq, Eq)]
pub enum CompatibilityLayer {
    NONE,
    TRANSLATE(TranslationLayer),
    WINE,
    BOTH(TranslationLayer),
}

pub struct Game {
    pub name: String,
    pub id: u64,
    pub supportlevel: SupportLevel,
    pub compatlayer: CompatibilityLayer,
}

impl Game {
    /// Creates a new [`Game`].
    fn new(
        name: String,
        id: u64,
        support_level: SupportLevel,
        compatibility_layer: CompatibilityLayer,
    ) -> Game {
        Game {
            name,
            id,
            supportlevel: support_level,
            compatlayer: compatibility_layer,
        }
    }
    fn grading(self) -> String {
        let grade: &str;
        match self.supportlevel {
            SupportLevel::PERFECT => grade = "S",
            SupportLevel::GREAT => grade = "A",
            SupportLevel::GOOD => grade = "B",
            SupportLevel::BAD => grade = "C",
            SupportLevel::FAIL => grade = "D",
        }
        let mut grade = grade.to_string();
        grade = match self.compatlayer {
            CompatibilityLayer::BOTH(ref _translation_layer) => grade.repeat(1),
            CompatibilityLayer::NONE => grade.repeat(3),
            _ => grade.repeat(2),
        };
        grade
    }
}

#[cfg(test)]
mod tests {
    use super::CompatibilityLayer;
    use super::Game;
    use super::SupportLevel;

    #[test]
    fn create_games() {
        let game = Game {
            name: "Test 1".to_string(),
            id: 1,
            supportlevel: SupportLevel::PERFECT,
            compatlayer: CompatibilityLayer::NONE,
        };
        assert_eq!(game.name, "Test 1");
        assert_eq!(game.id, 1);
    }

    #[test]
    fn games_grading() {
        let game = Game {
            name: "Test 1".to_string(),
            id: 1,
            supportlevel: SupportLevel::PERFECT,
            compatlayer: CompatibilityLayer::NONE,
        };
        let grade = game.grading();
        assert_eq!(grade, "SSS");

        let game = Game {
            name: "Test 2".to_string(),
            id: 2,
            supportlevel: SupportLevel::GREAT,
            compatlayer: CompatibilityLayer::WINE,
        };
        let grade = game.grading();
        assert_eq!(grade, "AA");
    }
}
