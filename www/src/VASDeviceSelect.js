import React, {useRef} from 'react';
import Modal from 'react-bootstrap/Modal';
import ListGroup from "react-bootstrap/ListGroup";
import Badge from "react-bootstrap/Badge";

export default function VASDeviceSelect(props) {
    if (!props.devices) {
        return null;
    }

    return <Modal
        show={!!props.open}
        onHide={props.onClose}
        backdrop="static"
        keyboard={false}
        centered
    >
        <Modal.Header closeButton>
            <Modal.Title>Select Tappybara device</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            <ListGroup variant="flush">
                {props.devices.map(v => {
                    const last_seen = v.last_seen ? new Date(Date.parse(v.last_seen)) : null;
                    return <ListGroup.Item key={v.id} disabled={!v.online} action={!!v.online} onClick={!!v.online ? () => {
                        props.onSelect(v.id);
                    } : null} className="d-flex justify-content-between align-items-start">
                        <div className="ms-2 me-auto">
                            <div className="fw-bold">{v.name}</div>
                            {last_seen ? `Last seen: ${last_seen.toLocaleString()}` : `Never online`}
                        </div>
                        {v.id === props.selectedDeviceId ? <Badge bg="success" pill>
                            Selected
                        </Badge> : null}
                    </ListGroup.Item>;
                })}
            </ListGroup>
        </Modal.Body>
    </Modal>
}