import { server } from './index';
// import './run_sql.ts';  // This will run the tasks in run_sql.ts

server.listen(3000, () => {
    console.log('Server is listening on port 3000');
});