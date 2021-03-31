use zoon::*;

mod element;
use element::counter::Counter;

blocks!{

    #[cmp]
    fn root() -> Cmp {
        Column::new()
            .item(control_counters())
            .item(counters())
    }

    #[cmp]
    fn control_counters() -> Cmp {
        Row::new()
            .item(column_counter())
            .item(row_counter())
            .item(counter_count())
            .item(counter_count_hundreds())
            .item(test_counters())
            .item(click_me_button())
    }

    #[cmp]
    fn click_me_button() -> Cmp {
        let title = cmp_var(|| "Click me!".to_owned());
        let click_count = cmp_var(|| 0);
        Row::new()
            .item(Button::new()
                .label(title.inner())
                .on_press(move || {
                    click_count.update(|count| count + 1);
                    title.set(format!("Clicked {}x", click_count.inner()));
                })
            )
    } 

    #[cmp]
    fn test_counters() -> Cmp {
        Row::new()
            .item("Test counters")
            .item(Counter::new()
                .value(super::test_counter_value().inner())
                .on_change(super::set_test_counter_value)
            )
            .item(Counter::new())
    } 

    #[cmp]
    fn counter_count() -> Cmp {
        El::new().child(format!("Counters: {}", super::counter_count().inner()))
    }

    #[cmp]
    fn counter_count_hundreds() -> Cmp {
        El::new().child(super::counter_count_hundreds().map(|count| {
            format!("Thousands: {}", count)
        }))
    }

    #[cmp]
    fn column_counter() -> Cmp {
        Row::new()
            .item("Columns:")
            .item(Counter::new()
                .value(super::column_count().inner())
                .on_change(super::set_column_count)
                .step(5)
            )
    }

    #[cmp]
    fn row_counter() -> Cmp {
        Row::new()
            .item("Rows:")
            .item(Counter::new()
                .value(super::row_count().inner())
                .on_change(super::set_row_count)
                .step(5)
            )
    }

    #[cmp]
    fn counters() -> Cmp {
        Column::new()
            .items((0..super::row_count().inner()).map(|_| counter_row()))
    }

    #[cmp]
    fn counter_row() -> Cmp {
        Row::new()
            .items((0..super::column_count().inner()).map(|_| counter()))
    }

    #[cmp]
    fn counter() -> Cmp {
        Counter::new()
    }

}