import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { ThemeProvider } from '@janis.me/react-themed/js';

import App from './app';

import './styles/index.scss';

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
createRoot(document.getElementById('root')!).render(
  <StrictMode>
        <ThemeProvider>
    <App />
    </ThemeProvider>
  </StrictMode>,
);
