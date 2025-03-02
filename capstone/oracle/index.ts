import { OracleJob } from "@switchboard-xyz/common";

// Define the cricket feed job
const jobConfig = [
    {
      tasks: [
        {
            httpTask: {
              url: "https://cricbuzz-cricket.p.rapidapi.com/matches/v1/recent",
              method: "GET",
              headers: {
                "x-rapidapi-key": "6536e062e4mshb0a7c8ddbc0335ap18ffefjsne13efcb9e118",
                "x-rapidapi-host": "cricbuzz-cricket.p.rapidapi.com",
              },
            },
          },
        {
          jsonParseTask: {
            path: "$.data[0]", // Get the first live match
          }
        },
        {
          jsonParseTask: {
            path: "{teamInfo, score, overs, status}",
          }
        }
      ],
    },
  ];


export function buildCricketJob(pair: string): OracleJob {
    jobConfig
    return OracleJob.fromObject(jobConfig);
  }
  

  
console.log("Fetching live cricket scores...\n");

// Serialize jobs to base64
const serializedJobs = jobConfig.map((oracleJob) => {
  const encoded = OracleJob.encodeDelimited(oracleJob).finish();
  return Buffer.from(encoded).toString("base64");
});

// Send job to Switchboard’s simulation server
async function fetchCricketData() {
  try {
    const response = await fetch("https://api.switchboard.xyz/api/simulate", {
      method: "POST",
      headers: [["Content-Type", "application/json"]],
      body: JSON.stringify({ cluster: "Mainnet", jobs: serializedJobs }),
    });

    if (response.ok) {
      const data = await response.json();
      console.log(`Live Cricket Feed:\n${JSON.stringify(data, null, 2)}`);
    } else {
      console.log(`Error (${response.status}): ${await response.text()}`);
    }
  } catch (error) {
    console.error("Failed to fetch cricket data:", error);
  }
}

fetchCricketData();
