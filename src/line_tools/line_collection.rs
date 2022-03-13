use super::line::Line;

pub fn create_sorted_ref_vec(text_lines: &Vec<Line>) -> Vec<&Line> {
    let mut text_lines_ref: Vec<&Line> = vec![];

    for r in text_lines {
        text_lines_ref.push(r);
    }
    text_lines_ref.sort_by_key(|k| &k.text);
    text_lines_ref
}

pub fn remove_duplicate_lines(remove_from: &mut Vec<Line>) {
    let remove_from_refs = create_sorted_ref_vec(&remove_from);
    let mut inxs_to_remove: Vec<usize> = vec![];
    let tlr_iter = remove_from_refs.iter().skip(1);
    let mut inx = 0;
    for r in tlr_iter {
        if r.get_text().eq(remove_from_refs[inx].get_text()) {
            inxs_to_remove.push(remove_from_refs[inx].line_number as usize);
        }
        inx += 1;
    }

    for i in inxs_to_remove {
        remove_from[i].drop()
    }
}

pub fn remove_lines_starts_with(remove_from: &mut Vec<Line>, remove_these: &Vec<&str>) {
    for line in remove_from {
        if !line.keep {
            continue;
        }
        for r in remove_these {
            if line.text.starts_with(r) {
                line.drop();
                continue;
            }
        }
    }
}
