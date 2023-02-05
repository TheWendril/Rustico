import React from 'react';
import {Route, BrowserRouter, Routes} from 'react-router-dom';
import Login from './Routes/login.js';

function Router(){

return(

    <BrowserRouter>
        <Routes>

            <Route component={<Login/>} path="/"/>

        </Routes>
    </BrowserRouter>
);

}

export default Router;