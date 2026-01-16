use std::fmt::{self, Display};

const AGED_BRIE: &str = "Aged Brie";
const BACKSTAGE_PASSES: &str = "Backstage passes to a TAFKAL80ETC concert";
const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";
const CONJURED: &str = "Conjured Mana Cake";

const QUALITY_LIMIT: i32 = 50;

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

/// Represents the kind of `Item`, each item kind behaves differently within the `GildedRose`.
pub enum ItemKind {
    Normal,
    Aging,
    Pass,
    Legendary,
}

impl From<&Item> for ItemKind {
    fn from(item: &Item) -> Self {
        match item.name.as_str() {
            AGED_BRIE => Self::Aging,
            BACKSTAGE_PASSES => Self::Pass,
            SULFURAS => Self::Legendary,
            _ => Self::Normal,
        }
    }
}

impl From<&mut Item> for ItemKind {
    fn from(item: &mut Item) -> Self {
        (&*item).into()
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            match item.into() {
                ItemKind::Normal => Self::update_normal_item_quality(item),
                ItemKind::Pass => Self::update_pass_item_quality(item),
                ItemKind::Aging => Self::update_aging_item_quality(item),
                // Legendary items never change
                ItemKind::Legendary => {}
            }
        }
    }

    fn update_normal_item_quality(item: &mut Item) {
        if item.sell_in > 0 {
            item.quality = (item.quality - 1).max(0);
        } else {
            item.quality = (item.quality - 2).max(0);
        }
        item.sell_in -= 1;
    }

    fn update_aging_item_quality(item: &mut Item) {
        if item.sell_in > 0 {
            item.quality = (item.quality + 1).min(QUALITY_LIMIT);
        } else {
            item.quality = (item.quality + 2).min(QUALITY_LIMIT);
        }
        item.sell_in -= 1;
    }

    fn update_pass_item_quality(item: &mut Item) {
        if item.sell_in > 10 {
            item.quality = (item.quality + 1).min(QUALITY_LIMIT);
        } else if item.sell_in > 5 {
            item.quality = (item.quality + 2).min(QUALITY_LIMIT);
        } else if item.sell_in > 0 {
            item.quality = (item.quality + 3).min(QUALITY_LIMIT);
        } else {
            item.quality = 0;
        }
        item.sell_in -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{AGED_BRIE, BACKSTAGE_PASSES, CONJURED, GildedRose, Item, QUALITY_LIMIT, SULFURAS};

    const DEX_VEST: &str = "+5 Dexterity Vest";

    #[test]
    fn sell_in_decrements_each_day() {
        let item = Item::new(DEX_VEST, 10, 30);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].sell_in, 9);
    }

    #[test]
    fn sulfuras_sell_in_never_changes() {
        let item = Item::new(SULFURAS, 10, 80);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].sell_in, 10);
    }

    #[test]
    fn quality_decrements_each_day() {
        let item = Item::new(DEX_VEST, 10, 30);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 29);
    }

    #[test]
    fn quality_always_non_negative() {
        let item = Item::new(DEX_VEST, 10, 0);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 0);
    }

    #[test]
    fn quality_always_within_upper_limit() {
        // Aged brie quality will increase with time
        let item = Item::new(AGED_BRIE, 10, QUALITY_LIMIT);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, QUALITY_LIMIT);
    }

    #[test]
    fn quality_decreases_2x_past_sell_date() {
        let item = Item::new(DEX_VEST, 0, 30);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 28);
    }

    #[test]
    fn aged_brie_quality_increases_with_time() {
        let item = Item::new(AGED_BRIE, 10, 30);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 31);
    }

    #[test]
    fn aged_brie_quality_increases_2x_after_sell_date() {
        let item = Item::new(AGED_BRIE, 0, 30);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 32);
    }

    #[test]
    fn sulfuras_quality_never_changes() {
        let item = Item::new(SULFURAS, 10, 80);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 80);
    }

    #[test]
    fn backstage_pass_quality_increments_more_than_10_days() {
        let item = Item::new(BACKSTAGE_PASSES, 11, 10);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 11);
    }

    #[test]
    fn backstage_pass_quality_increases_2x_within_10_days() {
        let item = Item::new(BACKSTAGE_PASSES, 10, 10);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();
        assert_eq!(gilded_rose.items[0].quality, 12);

        gilded_rose.update_quality();
        assert_eq!(gilded_rose.items[0].quality, 14);
    }

    #[test]
    fn backstage_pass_quality_increases_3x_less_than_5_days() {
        let item = Item::new(BACKSTAGE_PASSES, 5, 10);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();
        assert_eq!(gilded_rose.items[0].quality, 13);

        gilded_rose.update_quality();
        assert_eq!(gilded_rose.items[0].quality, 16);
    }

    #[test]
    fn backstage_pass_quality_0_after_sell_date() {
        let item = Item::new(BACKSTAGE_PASSES, 0, 10);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 0);
    }

    #[test]
    fn conjured_quality_decreases_2x_per_day_before_sell_date() {
        let item = Item::new(CONJURED, 10, 10);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 8);
    }

    #[test]
    fn conjured_quality_decreases_4x_per_day_after_sell_date() {
        let item = Item::new(CONJURED, 0, 10);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 6);
    }
}
