export const TransactionDirEnum = [
	'Request',
	'Offer'
];

export const TransactionDirIcons = [
	'exclamation-triangle',
	'hands-helping'
];

export const TransactionTypesEnum = [
	'Work',
    'Material',
    'Transport',
    'Production'
];

export const TransactionTypesIcons = [
	'briefcase',
	'parachute-box',
	'truck',
	'industry'
];

export const ConstraintType = [
	'briefcase',
	'parachute-box',
	'truck',
	'industry'
];

export const ConstraintTypeOpEnum = [
    'bool_eq',

    'list_all',
    'list_some',
    
    'numeric_eq',
    'numeric_leq',
    'numeric_geq',
    'numeric_gt',
    'numeric_lt'
];

export const ConstraintTypeOpRender = {
    bool_eq: '=',

    list_all: 'all',
    list_some: 'some',
	
	numeric_nq: '≠',
    numeric_eq: '=',
    numeric_leq: '≤',
    numeric_geq: '≥',
    numeric_gt: '>',
    numeric_lt: '<'
};