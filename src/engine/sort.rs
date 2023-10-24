pub struct MoveSort {
    mvs: Vec<isize>,
    vls: Vec<isize>,
    history: Vec<isize>,
    eng: super::core::Engine,
    index: usize,
    phase: super::pregen::Phase,
    signle: bool,
}

impl MoveSort {
    pub fn new(
        eng: super::core::Engine,
        killer_table: Vec<[isize; 2]>,
        history: Vec<isize>,
    ) -> Self {
        let mut sort = Self {
            mvs: vec![],
            vls: vec![],
            history,
            eng,
            index: 0,
            phase: super::pregen::Phase::HASH,
            signle: false,
        };

        if sort.eng.in_check() {
            sort.phase = super::pregen::Phase::REST;
            let all_mvs: Vec<isize> = sort.eng.generate_mvs(&mut None);
            for mv in all_mvs {
                if sort.eng.make_move(mv) {
                    continue;
                }
            }
        };

        sort
    }
}
