import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import { Map, Marker, Popup, TileLayer } from 'react-leaflet'
import { TransactionTypesIcons } from '../models/references'
import 'leaflet.icon.glyph';

import chroma from 'chroma-js';

import { useObservable } from 'react-use';

const useStyles = makeStyles((theme) => ({
  root: {
	flexGrow: 1,
	height: '50%'
  },
}));

const colorScale = chroma.scale(['#0b994b', '#d4111b']);

export const MapComponent = (props) => {
	const position = [59.3293, 18.0686];
	const data = useObservable(props.data, []);

	const classes = useStyles();
	return (
		<Map className={classes.root} center={position} zoom={13}>
			<TileLayer
			url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
			attribution="&copy; <a href=&quot;http://osm.org/copyright&quot;>OpenStreetMap</a> contributors"
			/>
			{/* <Marker position={position}> */}
			{data.map(row => {
				const scale = row.priority / 100
				return <Marker position={[row.lat, row.lng]} icon={window.L.icon.glyph({
					iconUrl: 'data:image/png;base64,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', 
					glyphColor: colorScale(scale || 0),
					prefix: 'fas',
					glyph: TransactionTypesIcons[row.transaction_type_id]
				})} />;
			})}
			{/* <Popup>A pretty CSS3 popup.<br />Easily customizable.</Popup> */}
			{/* </Marker> */}
		</Map>
	);
}