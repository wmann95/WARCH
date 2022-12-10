pub struct RAM {
    segments: Vec<Vec<u64>>,
    free_segs: Vec<usize>
}

impl RAM{
    pub fn new() -> Self{
        RAM{
            segments: Vec::new(),
            free_segs: Vec::new()
        }
    }

    pub fn request_segment(&mut self, size: usize) -> usize{
        let data = vec![0u64; size];

        if !self.free_segs.is_empty(){
            let id = self.free_segs.pop().unwrap();
            self.segments[id] = data;
            id
        }
        else {
            let id = self.segments.len();
            self.segments.push(data);
            id
        }
    }

    pub fn release_segment(&mut self, seg_id: usize){
        self.free_segs.push(seg_id);
    }
    
    pub fn duplicate_segment(&mut self, from: usize, to: usize){
        let buffer = self.segments[from].clone();
        self.segments[to] = buffer;
    }

    pub fn get(&mut self, seg_id: usize, index: usize) -> u64{
        self.segments[seg_id][index]
    }
    
    pub fn set(&mut self, seg_id: usize, index: usize, value: u64){
        self.segments[seg_id][index] = value;
    }
}