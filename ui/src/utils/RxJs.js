import { Observable } from 'rxjs';
import { useState, useEffect } from 'react';

export const useObservable = (observable, initialValue) => {
    const [state, setState] = useState(initialValue);
  
    useEffect(() => {
      const sub = observable.subscribe(setState);
      return () => sub.unsubscribe();
    }, [observable]);
  
    return state;
};