//! SmartTable helpers.

use fltk::prelude::*;
use fltk_table::{SmartTable, TableOpts};
use ndarray::Array2;

pub struct TableHelper;

impl TableHelper {
    /// A SmartTable configured as read-only, no headers, columns resizable.
    pub fn new_standard(x: i32, y: i32, w: i32, h: i32) -> SmartTable {
        let mut t = SmartTable::new(x, y, w, h, "").with_opts(TableOpts {
            rows: 0,
            cols: 0,
            editable: false,
            ..Default::default()
        });
        t.set_col_resize(true);
        t.set_col_header(false);
        t.set_row_header(false);
        t.end();
        t
    }

    /// Fill a SmartTable from an `Array2<String>` and distribute column widths.
    pub fn fill_from_array(table: &mut SmartTable, data: &Array2<String>) {
        let nrows = data.nrows() as i32;
        let ncols = data.ncols() as i32;
        let rows = nrows.max(1);
        let cols = ncols.max(1);

        table.set_opts(TableOpts {
            rows,
            cols,
            editable: false,
            ..Default::default()
        });

        for r in 0..nrows {
            for c in 0..ncols {
                let v = &data[[r as usize, c as usize]];
                table.set_cell_value(r, c, v);
            }
        }

        Self::auto_col_widths(table, cols);
        table.redraw();
    }

    /// Set the first row to the given headers.
    pub fn set_headers(table: &mut SmartTable, headers: &[&str]) {
        for (i, h) in headers.iter().enumerate() {
            table.set_cell_value(0, i as i32, h);
        }
    }

    /// Clear the table.
    pub fn clear(table: &mut SmartTable) {
        table.set_opts(TableOpts {
            rows: 0,
            cols: 0,
            editable: false,
            ..Default::default()
        });
        table.redraw();
    }

    /// Distribute the table width evenly across `cols` columns.
    pub fn auto_col_widths(table: &mut SmartTable, cols: i32) {
        if cols <= 0 {
            return;
        }
        let w = table.width();
        let cw = w / cols;
        for c in 0..cols {
            table.set_col_width(c, cw);
        }
    }
}