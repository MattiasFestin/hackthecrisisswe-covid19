import React, { useState, useRef } from 'react';

import Button from '@material-ui/core/Button';
import ButtonGroup from '@material-ui/core/ButtonGroup';
import Modal from '@material-ui/core/Modal';
import Typography from '@material-ui/core/Typography';
import Slider from '@material-ui/core/Slider';
import Paper from '@material-ui/core/Paper';
import FormControl from '@material-ui/core/FormControl';
import InputLabel from '@material-ui/core/InputLabel';
import Select from '@material-ui/core/Select';
import MenuItem from '@material-ui/core/MenuItem';
import TextField from '@material-ui/core/TextField';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'

import { makeStyles } from '@material-ui/core/styles';
import { Map, Marker, TileLayer, GeoJSON, Circle } from 'react-leaflet'
import chroma from 'chroma-js';
import { useObservable } from 'react-use';
import 'leaflet.icon.glyph';


import { TransactionTypesIcons, TransactionTypesEnum, TransactionDirEnum, TransactionDirIcons } from '../models/references'

const useStyles = makeStyles((theme) => ({
  root: {
	flexGrow: 1,
	height: '50%'
  },
  map: {
	flexGrow: 1,
	height: 'calc(100% - 30px)'
  },
  buttonGroup: {
	margin: 5
  },
  button: {
	flexGrow: 1,
	marginLeft: 20
  },
  modal: {
		width: 500,
		height: 600,
		top: `50%`,
		margin:'auto',
		// left: `50%`,
		// transform: `translate(50%, 50%)`,
		// position: 'fixed'
  },
  formControl: {
	margin: theme.spacing(1),
	minWidth: 120,
  },
  buttons: {
	margin: theme.spacing(1),
	minWidth: 120,
	'& > *': {
		margin: theme.spacing(1),
	},
  },
  filterSelect: {
	  width: 200
  }
}));

const colorScale = chroma.scale(['#0b994b', '#d4111b']);

export const MapComponent = (props) => {
	const position = [59.3293, 18.0686];
	const data = useObservable(props.transactionService.get$, []);
	const mapRef = useRef(null);
	const [open, setOpen] = useState(false);
	
	//New data
	const [dir, setDir] = useState(0);
	const [latlng, setLatlng] = useState({lat: 0, lng: 0});
	const [priority, setPriority] = useState(0);
	const [type, setType] = useState(0);
	const [description, setDescription] = useState('');

	const [showKomuner, setShowKomuner] = useState(false);
	const [showRegioner, setShowRegioner] = useState(true);

	const handleClose = () => {
		setOpen(false);
	};

	const handleSave = () => {
		const data = {
			transaction_direction_id: dir,
			transaction_type_id: type,
			priority,
			what: description,
			lat: latlng.lat,
			lng: latlng.lng,
			constraints: []
		};

		props.transactionService.exec.next({
			action: 'insert',
			data
		});
		setOpen(false);
	};

	if (props.komuner.features) {
		console.table(props.komuner.features[0].properties);
	}

	const classes = useStyles();
	return (
		<div className={classes.root}>
			<ButtonGroup color="primary" aria-label="outlined primary button group" className={classes.buttonGroup}>
				<Button color={showKomuner ? 'primary' : 'secondary'} className={classes.button} onClick={() => {setShowKomuner(!showKomuner)}}>Komuner</Button>
				<Button color={showRegioner ? 'primary' : 'secondary'} className={classes.button} onClick={() => {setShowRegioner(!showRegioner)}}>Regioner</Button>
			</ButtonGroup>
			&nbsp;&nbsp;<span>Region:</span>&nbsp;
			<Select
				className={classes.filterSelect}
				label="Description"
				value={'Stockholm'}
			>
				{props.regioner && props.regioner.features && props.regioner.features.map(f => <MenuItem key={'region' + f.properties.FA_kod} value={f.properties.FA_namn}>{f.properties.FA_namn}</MenuItem>)}
			</Select>
			&nbsp;&nbsp;<span>Komun:</span>&nbsp;
			<Select
				className={classes.filterSelect}
				label="Description"
			>
				{props.komuner && props.komuner.features && props.komuner.features.map(f => <MenuItem key={'komun' + f.properties.KnKod} value={f.properties.KnNamn}>{f.properties.KnNamn}</MenuItem>)}
			</Select>
			<Map
				ref={mapRef}
				className={classes.map}
				center={position}
				zoom={11}
				onclick={(e) => {
					// debugger;
					setOpen(true);
					setLatlng(e.latlng);
					setPriority(0);
					setDir(0);
					setDescription('');
					setType(0);
				}}
			>
				<TileLayer
					url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
					attribution="&copy; <a href=&quot;http://osm.org/copyright&quot;>OpenStreetMap</a> contributors"
				/>
				{/* <Marker position={position}> */}
				{data.map((row, index) => {
					const scale = row.priority / 100
					return <Marker key={index} position={[row.lat, row.lng]} icon={window.L.icon.glyph({
						iconUrl: 'data:image/png;base64,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', 
						glyphColor: colorScale(scale || 0),
						prefix: 'fas',
						glyph: TransactionTypesIcons[row.transaction_type_id]
					})} />;
				})}
				{data
					.filter(row => row.transaction_type_id === 2)
					.map((row, index) => {
						const scale = row.priority / 100
						return <Circle key={index} center={[row.lat, row.lng]} radius={10000} />;
					})}
				{showKomuner && props.komuner && props.komuner.features && props.komuner.features.map(f => <GeoJSON  opacity={0.5} color={chroma.random().darken()} key={'komun' + f.properties.KnKod} data={f} />)}
				{showRegioner && props.regioner && props.regioner.features && props.regioner.features.map(f => <GeoJSON  opacity={0.5} color={chroma.random().darken()} key={'region' + f.properties.FA_kod} data={f} />)}
			</Map>
			{/* <button type="button" onClick={handleOpen}>
			Open Modal
			</button> */}
			<Modal
				open={open}
				onClose={handleClose}
				aria-labelledby="simple-modal-title"
				aria-describedby="simple-modal-description"
				className={classes.modal}
				style={{display:'flex',alignItems:'center',justifyContent:'center'}}
			>
				<Paper style={{display: 'flex', flexWrap: 'wrap', width: '100%', height: '100%', padding: 20}}>
					<FormControl fullWidth className={classes.formControl}>
						<InputLabel>Direction</InputLabel>
						<Select
							value={dir}
							onChange={(e, newDir) => {
								setDir(newDir.props.value);
							}}
						>
							{TransactionDirEnum.map((text, index) => <MenuItem key={index} value={index}>
								<FontAwesomeIcon icon={TransactionDirIcons[index]} /> &nbsp; {text}	
							</MenuItem>)}
						</Select>
					</FormControl>
					<FormControl fullWidth className={classes.formControl}>
						<Typography id="discrete-slider" gutterBottom>
							Priority
						</Typography>
						<Slider
							defaultValue={0}
							valueLabelDisplay="auto"
							step={5}
							marks
							min={0}
							max={100}
							value={priority}
							onChange={(e, newPriority) => {
								setPriority(newPriority);
							}}
						/>
					</FormControl>
					<FormControl fullWidth className={classes.formControl}>
						<InputLabel>Type</InputLabel>
						<Select
							value={type}
							onChange={(e, newType) => {
								setType(newType.props.value);
							}}
						>
							{TransactionTypesEnum.map((text, index) => <MenuItem key={index} value={index}>
								<FontAwesomeIcon icon={TransactionTypesIcons[index]} /> &nbsp; {text}
							</MenuItem>)}
						</Select>
					</FormControl>
					<FormControl fullWidth className={classes.formControl}>
						<TextField
							label="Description"
							multiline
							rows="4"
							value={description}
							onChange={(e) => {
								setDescription(e.target.value);
							}}
						/>
					</FormControl>
					<FormControl fullWidth className={classes.formControl}>
						<TextField
							label="Position"
							// multiline
							// rows="4"
							value={latlng}
							readonly
							// onChange={(e, newDescription) => {
							// 	setDescription(newDescription);
							// }}
						/>
					</FormControl>
					<FormControl fullWidth className={classes.buttons}>
						<Button variant="contained" color="primary" onClick={handleSave}>
							Save
						</Button>
						<Button variant="contained" color="secondary" onClick={handleClose}>
							Abort
						</Button>
					</FormControl>
				</Paper>
			{/* {body} */}
			</Modal>
		</div>
	);
}