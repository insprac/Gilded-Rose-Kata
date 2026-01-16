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
        for i in 0..self.items.len() {
            if self.items[i].name != AGED_BRIE && self.items[i].name != BACKSTAGE_PASSES {
                if self.items[i].quality > 0 {
                    if self.items[i].name != SULFURAS {
                        self.items[i].quality = self.items[i].quality - 1;
                    }
                }
            } else {
                if self.items[i].quality < 50 {
                    self.items[i].quality = self.items[i].quality + 1;

                    if self.items[i].name == BACKSTAGE_PASSES {
                        if self.items[i].sell_in < 11 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }

                        if self.items[i].sell_in < 6 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }
                    }
                }
            }

            if self.items[i].name != SULFURAS {
                self.items[i].sell_in = self.items[i].sell_in - 1;
            }

            if self.items[i].sell_in < 0 {
                if self.items[i].name != AGED_BRIE {
                    if self.items[i].name != BACKSTAGE_PASSES {
                        if self.items[i].quality > 0 {
                            if self.items[i].name != SULFURAS {
                                self.items[i].quality = self.items[i].quality - 1;
                            }
                        }
                    } else {
                        self.items[i].quality = self.items[i].quality - self.items[i].quality;
                    }
                } else {
                    if self.items[i].quality < 50 {
                        self.items[i].quality = self.items[i].quality + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
}
