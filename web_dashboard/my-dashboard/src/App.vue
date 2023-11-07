<template>
  <div>
  <!-- Only show this section if no table is selected -->
    <div v-if="selectedTable === null && selectedDate === null">
      <div class="container">
        <!-- List of Databases -->
        <div class="left" v-if="selectedDatabase === null">
          <h2>Databases</h2>
          <ul>
            <li v-for="db in reversedDbs" :key="db" @click="selectDate(db)">
              {{ db }}
            </li>
          </ul>
        </div>

          <!-- Display the latest table -->
        <div class="right" v-if="latestTable && selectedDate == null">
            <h3>Most recent table:</h3>
            <p>Database: {{ latestTable.date }}</p>
            <p>Table: {{ latestTable.time }}</p>

            <!-- Display table contents -->
            <table>
                <thead>
                    <tr>
                        <!-- Assuming table content has columns: node_key, asset_name, path_value -->
                        <th>node_key</th>
                        <th>asset_name</th>
                        <th>path_value</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="row in latestTableContents" :key="row.id">
                        <td>{{ row.node_key }}</td>
                        <td>{{ row.asset_name }}</td>
                        <td>{{ row.path_value }}</td>
                    </tr>
                </tbody>
            </table>
        </div>
      </div>
    </div>

    <!-- List of Tables in a Selected Database -->
    <!-- <div v-else-if="selectedTable === null">
      <h2>Tables in {{ selectedDatabase }}</h2>
      <button @click="selectedDatabase = null">Back to databases</button>
      <ul>
        <li v-for="table in reversedTables" :key="table.name" @click="selectTable(table.name)">
          {{ table.name }} - {{ table.lastPathValue }}
        </li>
      </ul>
    </div> -->
      <div v-else-if="selectedTime === null">
        <h2>Tables in {{ selectedDate }}</h2>
        <button @click="selectedDate = null">Back to dates</button>
        <ul>
          <li v-for="table in reversedTables" :key="table" @click="selectTime(table)">
            <!-- {{ table }} - {{ table.lastPathValue }} -->
            {{ table }}
          </li>
        </ul>
      </div>
  </div>

    <!-- Display content of a selected table -->
      <div v-if="selectedTime">
        <button @click="selectedTime = null">Back to tables</button>
        <h2>Content of {{ selectedTime }}</h2>
        <table>
          <thead>
            <tr>
              <!-- Assuming table content has columns: col1, col2, col3 -->
              <th>node_key</th>
              <th>asset_name</th>
              <th>path_value</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in tableContent" :key="row.id">
              <td>{{ row.node_key }}</td>
              <td>{{ row.asset_name }}</td>
              <td>{{ row.path_value }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    
  
</template>

<script>
import axios from 'axios';
// const serverUrl = "https://f212-2603-6080-eb02-d337-d8c0-f24a-3e98-a1d9.ngrok.io"
// const serverUrl = "http://localhost:3000"
const serverUrl = "https://arb-server-a7fa597e65ca.herokuapp.com"
// const serverWs = "wss://f212-2603-6080-eb02-d337-d8c0-f24a-3e98-a1d9.ngrok.io"
// const serverWs = "ws://localhost:3000"
const serverWs = "wss://arb-server-a7fa597e65ca.herokuapp.com"

export default {
  data() {
    return {
      databases: [],
      tables: [],
      selectedDatabase: null,
      selectedTable: null,
      selectedTime: null,
      selectedDate: null,
      tableContent: [],
      latestTable: null,
      latestTableContents: [],
      ws: null
    };
  },
  methods: {
    setupWebSocket() {
      
      if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
        console.log('WebSocket connection already exists or is in the process of connecting.');
        return;
      }
      this.ws = new WebSocket(`${serverWs}`);

      // Set up WebSocket event listeners
      this.ws.onopen = (event) => {
        console.log('WebSocket connected:', event);
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
      };

      this.ws.addEventListener('message', (event) => {
        const message = JSON.parse(event.data);

        if (event.data === "TABLE_SUCCESS") {
          // console.log('New table created successfully');
          setTimeout(() => {
            this.fetchLatestTable();
            console.log('Latest table fetched');
          }, 5000);
        }

        if (message.type == 'update') {
          console.log('Received update message');
          setTimeout(() => {
            this.updateLatestTable(message.data);
          }, 1000);
        }
      });
      // Handle the WebSocket closing
      this.ws.onclose = (event) => {
        if (!event.wasClean) {
          console.error(`WebSocket error: ${event.code} ${event.reason}`);
        } else {
          console.log('WebSocket closed');
          // console.log(event)
        }
        // console.log('WebSocket closed: ' + event.code + ' ' + event.reason);
        // Reset the WebSocket instance to null
        this.ws = null;
      };
      // this.ws.addEventListener('open', (event) => {
      //   console.log('Connected to WebSocket server');
      //   console.log(event)
      // });

      // this.ws.addEventListener('error', (error) => {
      //   console.error('WebSocket Error:', error);
      // });

    },
     handleVisibilityChange() {
      if (document.visibilityState === 'hidden') {
        // Optionally close the WebSocket connection here
        if (this.ws) {
          this.ws.close();
        }
      } else if (document.visibilityState === 'visible') {
        // Re-establish the WebSocket connection
        this.setupWebSocket();
      }
    },
    selectDate(date) {
      this.selectedDate = date;
      axios.get(`${serverUrl}/getMongoDate?date=${this.selectedDate}`)
        .then(response => {
          this.tables = response.data;
        });
    },
    selectTime(time) {
      this.selectedTime = time;
      axios.get(`${serverUrl}/getMongoDateTime?date=${this.selectedDate}&time=${this.selectedTime}`)
        .then(response => {
          console.log(response.data[0])
          console.log(response.data[0].record)
          this.tableContent = response.data[0].record;
        });
    },
    fetchLatestTable() {
      axios.get(`${serverUrl}/getMongoLatest`).then(response => {
        this.latestTable = response.data;
        this.latestTableContents = response.data.record;
      });
      console.log("Latest table fetched")
    },
    updateLatestTable(tableData) {
      console.log("Updating latest table")
      console.log(tableData)
      console.log(tableData.date)
      console.log(tableData.time)
      this.latestTable = tableData;
      this.latestTableContents = tableData.record;
      console.log("Latest table updated")
    }
  },
  computed: {
    reversedTables() {
      return this.tables.slice().reverse();
    },
    reversedDbs() {
      return this.databases.slice().reverse();
    }
  },

  async mounted() {
    axios.get(`${serverUrl}/getMongoDates`).then(response => {
      this.databases = response.data;
    })
    .catch(error => {
      console.error("There was an error fetching the MONGODB data:", error);
    });

    this.fetchLatestTable();
    // this.ws = new WebSocket(`${serverWs}`);
    this.setupWebSocket()
    document.addEventListener('visibilitychange', this.handleVisibilityChange);



   
  },
  beforeUnmount() {
    // this.ws.close();
    document.removeEventListener('visibilitychange', this.handleVisibilityChange);
    if (this.ws) {
      this.ws.close(); // Close the WebSocket connection when the component is destroyed
    }
  }
}
</script>

<style scoped>
/* your CSS styles here */

/* Flex container */
.container {
  display: flex;
  justify-content: space-between;
  /* Add some space between the two child containers */
}

/* Left container (database list) */
.left {
  flex: 1;
  /* Take up available space */
  margin-right: 20px;
  /* Optional: Add some margin for spacing */
}

/* Right container (latest table) */
.right {
  flex: 2;
  /* Take up double the available space compared to the left */
}</style>