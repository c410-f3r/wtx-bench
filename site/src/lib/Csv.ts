import BenchStats from './BenchStats';
import { randomColor } from './Utils';
import type { firstPlacesChart, manyDatesChart } from './types';

type Dates = Map<number, Protocols>;
type Environments = Map<string, Dates>;
type Implementations = Map<string, Tests>;
type Protocols = Map<string, Implementations>;
type Tests = {
  geometricMean: number;
  tests: Map<string, BenchStats>;
};

class Csv {
  public results: Environments;

  constructor(data: string) {
    this.results = new Map();

    const lines = data.split('\n');
    lines.slice(1, -1).forEach((line) => {
      const values = line.split(',');
      const environment = values[0];
      const protocol = values[1];
      const test = values[2];
      const implementation = values[3];
      const timestamp = parseInt(values[4]);
      const min = values[5];
      const max = values[6];
      const mean = values[7];
      const sd = values[8];

      let dates = this.results.get(environment);
      if (dates === undefined) {
        dates = new Map();
        this.results.set(environment, dates);
      }

      let protocols = dates.get(timestamp);
      if (protocols === undefined) {
        protocols = new Map();
        dates.set(timestamp, protocols);
      }

      let implementations = protocols.get(protocol);
      if (implementations === undefined) {
        implementations = new Map();
        protocols.set(protocol, implementations);
      }

      let tests = implementations.get(implementation);
      if (tests === undefined) {
        tests = { geometricMean: 0, tests: new Map() };
        implementations.set(implementation, tests);
      }

      const testValue = tests.tests.get(test);
      if (testValue === undefined) {
        const bench_stats = new BenchStats(
          parseFloat(min),
          parseFloat(max),
          parseFloat(mean),
          parseFloat(sd)
        );
        tests.geometricMean = tests.geometricMean + Math.log2(bench_stats.mean);
        tests.tests.set(test, bench_stats);
      }
    });

    this.results.forEach((environment) => {
      environment.forEach((dates) => {
        dates.forEach((protocols) => {
          protocols.forEach((implementations) => {
            const rslt = implementations.geometricMean / implementations.tests.size;
            implementations.geometricMean = Math.pow(2, rslt);
          });
        });
      });
    });
  }

  *allDates(environment: string, lowerBound: Date): Generator<number> {
    for (const date of this.results.get(environment)!.keys()) {
      if (date > lowerBound.getTime()) {
        yield date;
      }
    }
  }

  chartsData(
    environment: string,
    dates: number[],
    protocol: string,
    implementation: string,
    test: string
  ): [firstPlacesChart | undefined, manyDatesChart] {
    const firstPlaces: firstPlacesChart = new Map();
    const scores: manyDatesChart = [];

    const manageColors = (implementation: string): string => {
      let value = firstPlaces.get(implementation);
      if (value === undefined) {
        value = [randomColor(), 0];
        firstPlaces.set(implementation, value);
      }
      return value[0];
    };

    const manageFirstPlace = (implementation: string) => {
      let value = firstPlaces.get(implementation);
      if (value === undefined) {
        value = [randomColor(), 1];
      } else {
        value[1] = value[1] + 1;
      }
      firstPlaces.set(implementation, value);
    };

    const manageScore = (color: string, idx: number, name: string, value: number) => {
      let score = scores[idx];
      if (score == undefined) {
        score = [name, color, []];
        scores.push(score);
      }
      score[2].push(value);
    };

    const manyImplementationsManyTests = () => {
      const bestTests = new Map<string, [string, number]>();
      dates.forEach((date) => {
        let idx = 0;
        this.results
          .get(environment)
          ?.get(date)
          ?.get(protocol)
          ?.forEach(({ geometricMean, tests }, implementationName) => {
            manageScore(manageColors(implementationName), idx, implementationName, geometricMean);
            idx = idx + 1;
            tests.forEach((benchStats, testName) => {
              let bestTest = bestTests.get(testName);
              if (bestTest == undefined) {
                bestTest = [implementationName, benchStats.mean];
                bestTests.set(testName, bestTest);
              } else if (benchStats.mean < bestTest[1]) {
                bestTests.set(testName, [implementationName, benchStats.mean]);
              }
            });
          });
        bestTests.forEach(([implementationName]) => {
          manageFirstPlace(implementationName);
        });
        bestTests.clear();
      });
    };

    const manyImplementationsOneTest = () => {
      dates.forEach((date) => {
        let idx = 0;
        this.results
          .get(environment)
          ?.get(date)
          ?.get(protocol)
          ?.forEach(({ tests }, implementationName) => {
            const benchStats = tests.get(test)!;
            manageScore(manageColors(implementationName), idx, implementationName, benchStats.mean);
            idx = idx + 1;
          });
      });
    };

    const oneImplementationManyTests = () => {
      dates.forEach((date) => {
        const protocols = this.results.get(environment)?.get(date);
        if (protocols === undefined) {
          return;
        }
        const localImplementation = protocols.get(protocol)?.get(implementation);
        if (localImplementation == undefined) {
          return;
        }
        let idx = 0;
        localImplementation.tests.forEach((benchStats, testName) => {
          manageScore(randomColor(), idx, testName, benchStats.mean);
          idx = idx + 1;
        });
      });
    };

    const oneImplementationOneTest = () => {
      dates.forEach((date) => {
        const protocols = this.results.get(environment)?.get(date);
        if (protocols === undefined) {
          return;
        }
        const benchStats = protocols.get(protocol)?.get(implementation)!.tests.get(test);
        if (benchStats == undefined) {
          return;
        }
        manageScore(manageColors(implementation), 0, test, benchStats.mean);
      });
    };

    if (implementation === '' && test === '') {
      manyImplementationsManyTests();
      return [firstPlaces, scores];
    } else if (implementation === '' && test !== '') {
      manyImplementationsOneTest();
      return [undefined, scores];
    } else if (implementation !== '' && test === '') {
      oneImplementationManyTests();
      return [undefined, scores];
    } else {
      oneImplementationOneTest();
      return [undefined, scores];
    }
  }

  environments(): IterableIterator<string> {
    return this.results.keys();
  }
}

export default Csv;
