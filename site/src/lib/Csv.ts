import BenchStats from './BenchStats';
import { short8601String } from './Utils';
import type { firstPlacesChart, manyDaysChart } from './types';

class Csv {
  public environments: Set<string>;
  public implementations: Set<string>;
  public protocols: Set<string>;
  public results: Map<
    string,
    Map<string, Map<string, Map<string, [number, Map<string, BenchStats>]>>>
  >;
  public tests: Set<string>;

  constructor(data: string) {
    this.environments = new Set();
    this.implementations = new Set();
    this.protocols = new Set();
    this.results = new Map();
    this.tests = new Set();

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

      this.environments.add(environment);
      this.implementations.add(implementation);
      this.protocols.add(protocol);
      this.tests.add(test);

      let environment_value = this.results.get(environment);
      if (environment_value === undefined) {
        environment_value = new Map();
        this.results.set(environment, environment_value);
      }

      let day = short8601String(new Date(timestamp));
      let day_value = environment_value.get(day);
      if (day_value === undefined) {
        day_value = new Map();
        environment_value.set(day, day_value);
      }

      let protocol_value = day_value.get(protocol);
      if (protocol_value === undefined) {
        protocol_value = new Map();
        day_value.set(protocol, protocol_value);
      }

      let implementation_value = protocol_value.get(implementation);
      if (implementation_value === undefined) {
        implementation_value = [1, new Map()];
        protocol_value.set(implementation, implementation_value);
      }

      let test_value = implementation_value[1].get(test);
      if (test_value === undefined) {
        let bench_stats = new BenchStats(
          parseFloat(min),
          parseFloat(max),
          parseFloat(mean),
          parseFloat(sd)
        );
        implementation_value[0] = implementation_value[0] * bench_stats.mean;
        implementation_value[1].set(test, bench_stats);
      }
    });

    this.results.forEach((environment) => {
      environment.forEach((day) => {
        day.forEach((protocol) => {
          protocol.forEach((implementation) => {
            implementation[0] = Math.pow(implementation[0], 1 / implementation[1].size);
          });
        });
      });
    });
  }

  chartsData(
    environment: string,
    days: string[],
    protocol: string,
    implementation: string,
    test: string
  ): [firstPlacesChart | undefined, manyDaysChart | undefined] {
    let firstPlaces = new Map<string, number>(
      [...this.implementations.values()].map((elem) => [elem, 0])
    );
    let scores: manyDaysChart = [];

    const manyImplementationsManyTests = () => {
      let bestTests = new Map<string, [string, number]>();
      days.forEach((day) => {
        this.results
          .get(environment)
          ?.get(day)
          ?.get(protocol)
          ?.forEach(([_, tests], implementationName) => {
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
        bestTests.forEach(([implementationName, _]) => {
          firstPlaces.set(implementationName, firstPlaces.get(implementationName)! + 1);
        });
        bestTests.clear();
      });
    };

    const manyImplementationsOneTest = () => {
      days.forEach((day) => {
        let idx = 0;
        let bestImplementation: [string, number] | undefined = undefined;
        this.results
          .get(environment)
          ?.get(day)
          ?.get(protocol)
          ?.forEach(([implementationMean, _], implementationName) => {
            if (bestImplementation == undefined) {
              bestImplementation = [implementationName, implementationMean];
            } else {
              if (implementationMean < bestImplementation[1]) {
                bestImplementation = [implementationName, implementationMean];
              }
            }

            let score = scores[idx];
            if (score == undefined) {
              score = [implementationName, []];
              scores.push(score);
            }
            score[1].push(implementationMean);
            idx = idx + 1;
          });
        if (bestImplementation == undefined) {
          return;
        }
        firstPlaces.set(bestImplementation[0], firstPlaces.get(bestImplementation[0])! + 1);
      });
    };

    const oneImplementationManyTests = () => {
      days.forEach((day) => {
        let protocols = this.results.get(environment)?.get(day);
        if (protocols === undefined) {
          return;
        }
        let tests = protocols.get(protocol)?.get(implementation);
        if (tests == undefined) {
          return;
        }

        let idx = 0;
        tests[1].forEach((benchStats, testName) => {
          let score = scores[idx];
          if (score == undefined) {
            score = [testName, []];
            scores.push(score);
          }
          score[1].push(benchStats.mean);
          idx = idx + 1;
        });
      });
    };

    const oneImplementationOneTest = () => {
      days.forEach((day) => {
        let protocols = this.results.get(environment)?.get(day);
        if (protocols === undefined) {
          return;
        }
        let tests = protocols.get(protocol)?.get(implementation);
        if (tests == undefined) {
          return;
        }
        let score = scores[0];
        if (score == undefined) {
          score = [implementation, []];
          scores.push(score);
        }
        score[1].push(tests[0]);
      });
    };

    if (implementation === '' && test === '') {
      manyImplementationsManyTests();
      return [firstPlaces, undefined];
    } else if (implementation === '' && test !== '') {
      manyImplementationsOneTest();
      return [firstPlaces, scores];
    } else if (implementation !== '' && test === '') {
      oneImplementationManyTests();
      return [undefined, scores];
    } else {
      oneImplementationOneTest();
      return [undefined, scores];
    }
  }
}

export default Csv;
