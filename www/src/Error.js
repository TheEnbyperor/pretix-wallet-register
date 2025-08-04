import React from 'react';
import Modal from 'react-bootstrap/Modal';
import Alert from 'react-bootstrap/Alert';

export default function Error(props) {
    return <Modal
        show={!!props.error}
        onHide={props.onClose}
        backdrop="static"
        keyboard={false}
        centered
    >
        <Modal.Header closeButton>
            <Modal.Title>Error</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            <Alert variant="danger">
                {props.error}
            </Alert>
        </Modal.Body>
    </Modal>
}