import React from 'react';
import {createBrowserRouter, RouterProvider} from 'react-router-dom';
import Login from './Routes/login.js';


const RouterList = createBrowserRouter([
    
    {
        path: '/',
        element: <Login/>
    }


]);

function Router(){

return(

    <RouterProvider router={RouterList} />
);

}

export default Router;