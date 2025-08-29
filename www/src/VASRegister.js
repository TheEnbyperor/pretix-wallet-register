import React, {useRef} from 'react';
import Modal from 'react-bootstrap/Modal';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';

export default function VASRegister(props) {
    const serverRef = useRef(null);
    const tokenRef = useRef(null);

    if (!props.open) {
        return null;
    }

    const formSubmit = (evt) => {
        evt.preventDefault();
        if (!serverRef.current.value || !tokenRef.current.value) {
            return;
        }
        props.onValue(serverRef.current.value, tokenRef.current.value);
    }

    return <Modal
        show={!!props.open}
        onHide={props.onClose}
        backdrop="static"
        keyboard={false}
        centered
    >
        <Modal.Header closeButton>
            <Modal.Title>Register to Tappybara</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            <Form onSubmit={formSubmit}>
                <Form.Group className="mb-3">
                    <Form.Label>Server URL</Form.Label>
                    <Form.Control type="text" ref={serverRef} required />
                </Form.Group>
                <Form.Group className="mb-3">
                    <Form.Label>Registration token</Form.Label>
                    <Form.Control type="text" ref={tokenRef} required />
                </Form.Group>
                <Button variant="primary" type="submit">
                    Register
                </Button>
            </Form>
        </Modal.Body>
    </Modal>
}