use std::fmt::{self, Display};

const AGED_BRIE: &str = "Aged Brie";
const BACKSTAGE_PASSES: &str = "Backstage passes to a TAFKAL80ETC concert";
const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";

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

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name == SULFURAS {
                // Legendary items never change
                continue;
            }

            if item.name != AGED_BRIE && item.name != BACKSTAGE_PASSES {
                if item.quality > 0 {
                    item.quality -= 1;
                }
            } else {
                if item.quality < 50 {
                    item.quality += 1;

                    if item.name == BACKSTAGE_PASSES {
                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.quality += 1;
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.quality += 1;
                            }
                        }
                    }
                }
            }

            if item.sell_in <= 0 {
                if item.name != AGED_BRIE {
                    if item.name != BACKSTAGE_PASSES {
                        if item.quality > 0 {
                            if item.name != SULFURAS {
                                item.quality -= 1;
                            }
                        }
                    } else {
                        item.quality = 0;
                    }
                } else {
                    if item.quality < 50 {
                        item.quality += 1;
                    }
                }
            }

            item.sell_in -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gildedrose::{BACKSTAGE_PASSES, SULFURAS};

    use super::{AGED_BRIE, GildedRose, Item};

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
        let item = Item::new(AGED_BRIE, 10, 50);
        let mut gilded_rose = GildedRose::new(vec![item]);

        gilded_rose.update_quality();

        assert_eq!(gilded_rose.items[0].quality, 50);
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
}
