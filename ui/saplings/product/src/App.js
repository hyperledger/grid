/**
 * Copyright 2018-2020 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import React from 'react';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import { library } from '@fortawesome/fontawesome-svg-core';
import {
  faCaretUp,
  faCaretDown,
  faCheck,
  faPenSquare,
  faChevronLeft,
  faPlus,
  faTimes,
  faSpinner,
  faTrashAlt
} from '@fortawesome/free-solid-svg-icons';
import { ToastProvider } from 'react-toast-notifications';

import { ServiceProvider } from './state/service-context';
import TopBar from './components/TopBar';
import ProductsTable from './components/ProductsTable';
import ProductInfo from './components/ProductInfo';
import './App.scss';

library.add(
  faCaretUp,
  faCaretDown,
  faCheck,
  faPenSquare,
  faChevronLeft,
  faPlus,
  faTimes,
  faSpinner,
  faTrashAlt
);

function App() {
  return (
    <ServiceProvider>
      <ToastProvider>
        <div id="product-sapling" className="product-app">
          <TopBar saplingName="Product Management" />
          <Router>
            <Switch>
              <Route exact path="/product">
                <ProductsTable />
              </Route>
              <Route path="/product/products/:id">
                <ProductInfo />
              </Route>
            </Switch>
          </Router>
        </div>
      </ToastProvider>
    </ServiceProvider>
  );
}

export default App;
