use enumflags2::{bitflags, BitFlags};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Value;
#[derive(PartialEq, Eq, Debug, Clone, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum SupportLevel {
    PERFECT = 0,
    GREAT = 1,
    GOOD = 2,
    BAD = 3,
    FAIL = 4,
}

#[bitflags]
#[repr(u32)]
#[derive(PartialEq, Eq, DeriveActiveEnum, EnumIter, Debug, Clone, Copy)]
#[sea_orm(rs_type = "u32", db_type = "Integer")]
pub enum CompatibilityLayerItem {
    WINE = 1,
    LATX = 2,
    LATA = 4,
    BOX64 = 8,
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone, Copy)]
pub struct Compatibility(pub BitFlags<CompatibilityLayerItem>);

impl std::convert::From<Compatibility> for Value {
    fn from(source: Compatibility) -> Self {
        source.0.bits().into()
    }
}

impl sea_orm::TryGetable for Compatibility {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let value = <u32 as sea_orm::TryGetable>::try_get_by(res, index).unwrap();
        let layer: BitFlags<CompatibilityLayerItem> = BitFlags::from_bits(value).unwrap();
        let result = Compatibility { 0: layer };
        Ok(result)
    }
}

impl sea_orm::sea_query::ValueType for Compatibility {
    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        let value = <u32 as sea_orm::sea_query::ValueType>::try_from(v).unwrap();
        let layer: BitFlags<CompatibilityLayerItem> = BitFlags::from_bits(value).unwrap();
        let result = Compatibility { 0: layer };
        Ok(result)
    }
    fn type_name() -> String {
        stringify!(Compatibility).to_owned()
    }
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Unsigned
    }
    fn column_type() -> ColumnType {
        sea_orm::sea_query::ColumnType::Unsigned
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "games")]
pub struct Model {
    pub name: String,
    #[sea_orm(primary_key)]
    pub id: u64,
    pub supportlevel: SupportLevel,
    pub compat: Compatibility,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Creates a new [`Model`].
    fn new(
        name: String,
        id: u64,
        support_level: SupportLevel,
        compatibility: Compatibility,
    ) -> Model {
        Model {
            name,
            id,
            supportlevel: support_level,
            compat: compatibility,
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
        if self.compat.0.is_empty() {
            // 不需要任何兼容层
            grade = grade.repeat(3);
        } else if self.compat.0 == CompatibilityLayerItem::WINE {
            // 只需要wine，但是这种程序应该不存在
            grade = grade.repeat(2)
        } else if !(self.compat.0.contains(CompatibilityLayerItem::WINE))
            && !(self.compat.0.is_empty())
        {
            // 只需要转译工具
            grade = grade.repeat(2)
        }
        grade
    }
}

#[cfg(test)]
mod tests {
    use enumflags2::make_bitflags;
    use enumflags2::BitFlags;

    use crate::entity::game::CompatibilityLayerItem;

    use super::Compatibility;
    use super::Model;
    use super::SupportLevel;

    #[test]
    fn create_games() {
        let game = Model {
            name: "Test 1".to_string(),
            id: 1,
            supportlevel: SupportLevel::PERFECT,
            compat: Compatibility {
                0: BitFlags::default(),
            },
        };
        assert_eq!(game.name, "Test 1");
        assert_eq!(game.id, 1);
        assert_eq!(game.supportlevel, SupportLevel::PERFECT);
        assert!(game.compat.0.is_empty())
    }

    #[test]
    fn games_grading() {
        let game = Model {
            name: "Test 1".to_string(),
            id: 1,
            supportlevel: SupportLevel::PERFECT,
            compat: Compatibility {
                0: BitFlags::default(),
            },
        };
        let grade = game.grading();
        assert_eq!(grade, "SSS");

        let game = Model {
            name: "Test 2".to_string(),
            id: 2,
            supportlevel: SupportLevel::GREAT,
            compat: Compatibility {
                0: make_bitflags!(CompatibilityLayerItem::{WINE}),
            },
        };
        let grade = game.grading();
        assert_eq!(grade, "AA");

        let game = Model {
            name: "Test 3".to_string(),
            id: 3,
            supportlevel: SupportLevel::GOOD,
            compat: Compatibility {
                0: make_bitflags!(CompatibilityLayerItem::{WINE | BOX64}),
            },
        };
        let grade = game.grading();
        assert_eq!(grade, "B");

        let game = Model {
            name: "Test 4".to_string(),
            id: 4,
            supportlevel: SupportLevel::BAD,
            compat: Compatibility {
                0: make_bitflags!(CompatibilityLayerItem::{LATX}),
            },
        };
        let grade = game.grading();
        assert_eq!(grade, "CC");

        let game = Model {
            name: "Test 5".to_string(),
            id: 5,
            supportlevel: SupportLevel::FAIL,
            compat: Compatibility {
                0: make_bitflags!(CompatibilityLayerItem::{LATA}),
            },
        };
        let grade = game.grading();
        assert_eq!(grade, "DD");
    }
}