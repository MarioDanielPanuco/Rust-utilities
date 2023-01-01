fn min_cost(distance: Vec<i32>, cost: Vec<f32>) -> f32 {
    let mut min_c: Vec<f32> = vec![0.0; 5];
    println!("{:?}", min_c);
    let n = cost.len();
    for i in n - 2..1 {
        min_c[i] = 1000.0;
        let mut j = i + 1;
        let diff = distance[j] - distance[i];
        while j <= n && diff <= 100 {
            min_c[i] = f32::min(min_c[i], cost[i] + min_c[j]);
            j += 1;
        }
    }
    min_c[1]
}

#[cfg(test)]
mod test {
    use crate::optimizers::fuel_cost::min_cost;

    #[test]
    fn test() {
        let x = min_cost(vec![0,98,134,186, 253], vec![5.0,6.0,2.4,12.0,7.2]);

        assert_eq!(x, 18.0);
    }
}