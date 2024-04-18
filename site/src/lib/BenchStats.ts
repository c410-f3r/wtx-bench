class BenchStats {
  public max: number;
  public mean: number;
  public min: number;
  public sd: number;

  constructor(min: number, max: number, mean: number, sd: number) {
    this.max = max;
    this.mean = mean;
    this.min = min;
    this.sd = sd;
  }
}

export default BenchStats;
